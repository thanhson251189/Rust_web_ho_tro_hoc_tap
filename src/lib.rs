pub mod html;
pub mod lessons;
pub mod store;

use crate::html::Flash;
use crate::lessons::{by_id, for_subject, Lesson, Subject};
use crate::store::{
    add_profile, list_profiles, next_lesson_id, record_answer, upsert_user, Profile, StoreError,
    MAX_PROFILES_PER_USER,
};
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Form, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};

pub const DEFAULT_DB_PATH: &str = "data/app.sqlite";
const LOCAL_PARENT_SUB: &str = "local-dev";
const AVATARS: &[&str] = &["robot", "cat", "bear", "fox"];

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Connection>>,
    user_id: i64,
}

impl AppState {
    pub fn in_memory() -> Result<Self, StoreError> {
        Self::from_conn(store::open_memory()?)
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, StoreError> {
        Self::from_conn(store::open_file(path)?)
    }

    fn from_conn(conn: Connection) -> Result<Self, StoreError> {
        let user = upsert_user(&conn, LOCAL_PARENT_SUB, "local@family")?;
        Ok(Self {
            user_id: user.id,
            db: Arc::new(Mutex::new(conn)),
        })
    }

    /// One panic while holding the lock must not take the whole app down: the
    /// connection is reused even if the lock was poisoned. `record_answer_at`
    /// writes through a transaction, so a panic rolls back to a consistent DB.
    fn db(&self) -> MutexGuard<'_, Connection> {
        self.db
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn profiles(&self) -> Vec<Profile> {
        let db = self.db();
        list_profiles(&db, self.user_id).expect("list profiles")
    }

    fn profile(&self, id: i64) -> Option<Profile> {
        self.profiles().into_iter().find(|profile| profile.id == id)
    }

    fn done_lesson_ids(&self, profile_id: i64) -> std::collections::HashSet<u32> {
        let db = self.db();
        crate::store::progress_for_profile(&db, profile_id)
            .expect("read progress")
            .into_iter()
            .filter(|p| p.done)
            .map(|p| p.lesson_id)
            .collect()
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/profiles", get(profiles_page).post(create_profile))
        .route("/profiles/:id", get(open_profile))
        .route("/profiles/:id/bao-cao", get(report_page))
        .route("/profiles/:id/mon/:slug", get(subject_page))
        .route(
            "/profiles/:id/bai/:lesson_id",
            get(lesson_page).post(answer_lesson),
        )
        .with_state(state)
}

async fn home() -> Redirect {
    Redirect::to("/profiles")
}

async fn profiles_page(State(state): State<AppState>) -> Html<String> {
    let profiles = state.profiles();
    Html(html::picker(
        &profiles,
        None,
        profiles.len() < MAX_PROFILES_PER_USER,
    ))
}

#[derive(Deserialize)]
struct NewProfile {
    name: String,
    avatar_key: String,
}

async fn create_profile(
    State(state): State<AppState>,
    Form(form): Form<NewProfile>,
) -> Html<String> {
    let name = form.name.trim().to_string();
    let avatar_key = if AVATARS.contains(&form.avatar_key.as_str()) {
        form.avatar_key
    } else {
        "robot".to_string()
    };

    let db = state.db();
    let error = if name.is_empty() {
        Some("Nhập tên hồ sơ.".to_string())
    } else {
        match add_profile(&db, state.user_id, &name, &avatar_key) {
            Ok(_) => None,
            Err(StoreError::ProfileLimit) => Some("Tối đa 2 hồ sơ.".to_string()),
            Err(err) => Some(format!("Không lưu được hồ sơ: {err:?}")),
        }
    };
    let profiles = list_profiles(&db, state.user_id).expect("list profiles");
    drop(db);
    Html(html::picker(
        &profiles,
        error.as_deref(),
        profiles.len() < MAX_PROFILES_PER_USER,
    ))
}

async fn open_profile(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    match state.profile(id) {
        Some(profile) => {
            let stars = {
                let db = state.db();
                crate::store::total_stars(&db, profile.id).unwrap_or(0)
            };
            Html(html::home(&profile, stars))
        }
        None => Html(html::missing_profile()),
    }
}

async fn subject_page(
    State(state): State<AppState>,
    Path((id, slug)): Path<(i64, String)>,
) -> Html<String> {
    let Some(profile) = state.profile(id) else {
        return Html(html::missing_profile());
    };
    let Some(subject) = Subject::parse(&slug) else {
        return Html(html::missing_subject());
    };
    let lessons = for_subject(subject);
    let done = state.done_lesson_ids(profile.id);
    let start_id = {
        let db = state.db();
        let ids: Vec<u32> = lessons.iter().map(|l| l.id).collect();
        next_lesson_id(&db, profile.id, &ids).expect("next lesson")
    };
    Html(html::subject_page(
        &profile, subject, &lessons, &done, start_id,
    ))
}

async fn report_page(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let Some(profile) = state.profile(id) else {
        return Html(html::missing_profile());
    };
    let db = state.db();
    let progress = crate::store::progress_for_profile(&db, profile.id).expect("read progress");
    let days = crate::store::daily_activity(&db, profile.id, 14).expect("daily activity");
    drop(db);

    let per_subject: Vec<(Subject, usize, usize)> = Subject::all()
        .iter()
        .map(|subject| {
            let total = for_subject(*subject).len();
            let done = progress
                .iter()
                .filter(|p| p.done && by_id(p.lesson_id).is_some_and(|l| l.subject == *subject))
                .count();
            (*subject, done, total)
        })
        .collect();
    let mut hard: Vec<&Lesson> = progress
        .iter()
        .filter(|p| p.wrong_count >= 2 && !p.done)
        .filter_map(|p| by_id(p.lesson_id))
        .collect();
    hard.sort_by_key(|l| l.id);
    let hard: Vec<&Lesson> = hard.into_iter().take(8).collect();
    Html(html::report_page(&profile, &per_subject, &days, &hard))
}

#[derive(Deserialize)]
struct LessonQuery {
    kq: Option<String>,
    tiep: Option<String>,
}

async fn lesson_page(
    State(state): State<AppState>,
    Path((id, lesson_id)): Path<(i64, u32)>,
    Query(query): Query<LessonQuery>,
) -> Html<String> {
    let flash = result_flash(&state, id, lesson_id, &query);
    render_lesson(&state, id, lesson_id, flash)
}

/// `kq=dung` is a display hint. The success banner is shown only after this
/// profile has actually finished the lesson. `tiep` is kept when it names a
/// real lesson in the same subject; otherwise the next unfinished lesson is used.
fn result_flash(
    state: &AppState,
    profile_id: i64,
    lesson_id: u32,
    query: &LessonQuery,
) -> Option<Flash> {
    match query.kq.as_deref() {
        Some("sai") => Some(Flash::Wrong),
        Some("dung") => {
            let tiep = parse_tiep(query.tiep.as_deref());
            finished_correct_flash(state, profile_id, lesson_id, tiep)
        }
        _ => None,
    }
}

/// A stale or hand-edited link must not 400 the page, so a `tiep` that does not
/// parse is dropped and the next unfinished lesson is used instead. Whether the
/// parsed id is a real lesson of the same subject is still checked later.
fn parse_tiep(raw: Option<&str>) -> Option<u32> {
    raw.and_then(|value| value.trim().parse::<u32>().ok())
}

fn finished_correct_flash(
    state: &AppState,
    profile_id: i64,
    lesson_id: u32,
    tiep: Option<u32>,
) -> Option<Flash> {
    let lesson = by_id(lesson_id)?;
    let done = state.done_lesson_ids(profile_id);
    if !done.contains(&lesson_id) {
        return None;
    }
    let next_id = match tiep {
        Some(id) if lesson_in_subject(id, lesson.subject) => Some(id),
        _ => next_unfinished_id(&for_subject(lesson.subject), lesson_id, &done),
    };
    Some(Flash::Correct { next_id })
}

fn lesson_in_subject(lesson_id: u32, subject: Subject) -> bool {
    by_id(lesson_id).is_some_and(|lesson| lesson.subject == subject)
}

#[derive(Deserialize)]
struct Answer {
    choice: usize,
}

async fn answer_lesson(
    State(state): State<AppState>,
    Path((id, lesson_id)): Path<(i64, u32)>,
    Form(form): Form<Answer>,
) -> Response {
    // The profile id comes from the URL, so it must be checked against this
    // parent's own profiles before anything is written to it.
    if state.profile(id).is_none() {
        return Html(html::missing_profile()).into_response();
    }
    let Some(lesson) = by_id(lesson_id) else {
        return Html(html::missing()).into_response();
    };
    let correct = form.choice == lesson.correct;
    {
        let db = state.db();
        if record_answer(&db, id, lesson_id, correct).is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(html::write_failed()),
            )
                .into_response();
        }
    }
    let flash = if correct {
        let mut done = state.done_lesson_ids(id);
        done.insert(lesson_id);
        Flash::Correct {
            next_id: next_unfinished_id(&for_subject(lesson.subject), lesson_id, &done),
        }
    } else {
        Flash::Wrong
    };
    // 303 so a refresh loads the result URL instead of submitting the form again.
    // The body stays so existing callers that read the POST response still see the flash.
    let location = match flash {
        Flash::Correct {
            next_id: Some(next),
        } => format!("/profiles/{id}/bai/{lesson_id}?kq=dung&tiep={next}"),
        Flash::Correct { next_id: None } => format!("/profiles/{id}/bai/{lesson_id}?kq=dung"),
        Flash::Wrong => format!("/profiles/{id}/bai/{lesson_id}?kq=sai"),
    };
    (
        StatusCode::SEE_OTHER,
        [(header::LOCATION, location)],
        render_lesson(&state, id, lesson_id, Some(flash)),
    )
        .into_response()
}

/// Next unfinished lesson in subject order after `current_id`.
/// If none remain later, the earliest unfinished lesson before it.
/// `None` when every lesson in the subject is done.
fn next_unfinished_id(
    subject_lessons: &[&Lesson],
    current_id: u32,
    done: &HashSet<u32>,
) -> Option<u32> {
    let pos = subject_lessons
        .iter()
        .position(|item| item.id == current_id)?;
    subject_lessons[pos + 1..]
        .iter()
        .chain(subject_lessons[..pos].iter())
        .find(|item| !done.contains(&item.id))
        .map(|item| item.id)
}

fn render_lesson(
    state: &AppState,
    profile_id: i64,
    lesson_id: u32,
    flash: Option<Flash>,
) -> Html<String> {
    let Some(profile) = state.profile(profile_id) else {
        return Html(html::missing_profile());
    };
    let Some(lesson) = by_id(lesson_id) else {
        return Html(html::missing());
    };
    let lessons = for_subject(lesson.subject);
    let index = lessons
        .iter()
        .position(|item| item.id == lesson.id)
        .unwrap_or(0);
    let stars = {
        let db = state.db();
        crate::store::total_stars(&db, profile_id).unwrap_or(0)
    };
    Html(html::lesson_page(
        &profile,
        lesson,
        index,
        lessons.len(),
        stars,
        flash,
    ))
}

#[cfg(test)]
mod tests {
    use super::{add_profile, app, next_unfinished_id, upsert_user, AppState};
    use crate::lessons::{for_subject, Subject};
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use std::collections::HashSet;
    use tower::ServiceExt;

    fn test_app() -> axum::Router {
        app(AppState::in_memory().unwrap())
    }

    async fn body_of(response: axum::response::Response) -> String {
        let bytes = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn form_type() -> &'static str {
        concat!("application/x-www-form-", "urlencoded")
    }

    async fn add_an(app: axum::Router) -> axum::Router {
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type())
                    .body(Body::from("name=An&avatar_key=robot"))
                    .unwrap(),
            )
            .await
            .unwrap();
        app
    }

    #[tokio::test]
    async fn home_redirects_to_profiles() {
        let response = test_app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.headers()["location"], "/profiles");
    }

    #[tokio::test]
    async fn picker_starts_empty() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/profiles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Ai đang học?"));
        assert!(html.contains("Thêm hồ sơ"));
    }

    #[tokio::test]
    async fn create_two_profiles_then_reject_third() {
        let app = test_app();

        let first = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type())
                    .body(Body::from("name=An&avatar_key=robot"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(first).await;
        assert!(html.contains("An"));

        let second = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type())
                    .body(Body::from("name=Binh&avatar_key=cat"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(second).await;
        assert!(html.contains("An"));
        assert!(html.contains("Binh"));
        assert!(!html.contains("Thêm hồ sơ"));

        let third = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type())
                    .body(Body::from("name=Chi&avatar_key=bear"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(third).await;
        assert!(html.contains("Tối đa 2 hồ sơ."));
        assert!(!html.contains("Chi"));
    }

    #[tokio::test]
    async fn profile_home_shows_three_subjects() {
        let app = add_an(test_app()).await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(response).await;
        assert!(html.contains("Xin chào, An"));
        assert!(html.contains("Toán"));
        assert!(html.contains("Tiếng Việt"));
        assert!(html.contains("Tiếng Anh"));
    }

    #[tokio::test]
    async fn unknown_profile_keeps_profile_copy() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/profiles/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Không có hồ sơ này"));
        assert!(!html.contains("Không có trang này"));
    }

    #[tokio::test]
    async fn unknown_subject_slug_is_not_a_subject() {
        let app = add_an(test_app()).await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/mon/nope")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Không có môn này"));
        assert!(!html.contains("Bắt đầu bài 1"));
    }

    #[tokio::test]
    async fn math_subject_lists_units_and_start_cta() {
        let app = add_an(test_app()).await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/mon/toan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Học tiếp"));
        assert!(html.contains("/profiles/1/bai/1"));
        assert!(html.contains("Đã học: <b>0</b>/62 bài"));
        assert!(html.contains("62 bài"));
        assert!(html.contains("Các số từ 0 đến 10"));
        assert!(html.contains("Cộng trừ trong phạm vi 10"));
        assert!(html.contains("Các số đến 100"));
        assert!(html.contains("Bài 1. Số 0 đến 5"));
        assert!(!html.contains("aria-label='đã học xong'"));
    }

    #[tokio::test]
    async fn finishing_lesson_marks_done_and_moves_cta() {
        let app = add_an(test_app()).await;
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/mon/toan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(response).await;
        assert!(html.contains("done-mark"));
        assert!(html.contains("Đã học: <b>1</b>/62 bài"));
        // "Học tiếp" now points at lesson 2, the first unfinished one
        assert!(html.contains("/profiles/1/bai/2"));
    }

    #[tokio::test]
    async fn wrong_answer_does_not_mark_done() {
        let app = add_an(test_app()).await;
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=1"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/mon/toan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(response).await;
        assert!(!html.contains("aria-label='đã học xong'"));
        assert!(html.contains("Đã học: <b>0</b>/62 bài"));
        assert!(html.contains("/profiles/1/bai/1"));
    }

    #[tokio::test]
    async fn stars_accumulate_and_show_on_home_and_lesson() {
        let app = add_an(test_app()).await;
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();

        let home = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/profiles/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(home).await;
        assert!(html.contains("⭐ 1 sao"));

        let lesson = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/bai/2")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(lesson).await;
        assert!(html.contains("sao thưởng"));
        assert!(html.contains("⭐ 1"));
    }

    #[tokio::test]
    async fn report_shows_progress_and_daily_activity() {
        let app = add_an(test_app()).await;
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/bao-cao")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Báo cáo học tập"));
        assert!(html.contains("1/62"));
        assert!(html.contains("0/80"));
        assert!(html.contains("0/59"));
        assert!(html.contains("✓ 1 bài"));
        assert!(html.contains("1 lượt trả lời"));
        assert!(html.contains("Bài bé hay sai"));
    }

    #[tokio::test]
    async fn report_lists_hard_lessons_after_repeated_wrong() {
        let app = add_an(test_app()).await;
        for _ in 0..2 {
            let _ = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/profiles/1/bai/1")
                        .header("content-type", form_type())
                        .body(Body::from("choice=1"))
                        .unwrap(),
                )
                .await
                .unwrap();
        }
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/profiles/1/bao-cao")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(response).await;
        assert!(html.contains("hay sai ✗"));
        assert!(html.contains("Bài 1. Số 0 đến 5"));
    }

    #[tokio::test]
    async fn unknown_profile_report_is_missing_profile() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/profiles/999/bao-cao")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(response).await;
        assert!(html.contains("Không có hồ sơ này"));
    }

    #[tokio::test]
    async fn math_lesson_accepts_correct_answer() {
        let app = add_an(test_app()).await;
        // Lesson 1 correct is index 0 ("3"); index 1 is a distractor.
        let wrong = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=1"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(wrong).await;
        assert!(html.contains("Chưa đúng"));

        let right = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(right).await;
        assert!(html.contains("Giỏi quá"));
        assert!(html.contains("/profiles/1/bai/2"));
    }

    #[tokio::test]
    async fn correct_answer_skips_already_finished_next_lesson() {
        let app = add_an(test_app()).await;
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/2")
                    .header("content-type", form_type())
                    .body(Body::from("choice=1"))
                    .unwrap(),
            )
            .await
            .unwrap();

        let right = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(right).await;
        assert!(html.contains("/profiles/1/bai/3"));
        assert!(!html.contains("/profiles/1/bai/2"));
    }

    async fn get_html(router: &axum::Router, uri: &str) -> String {
        let response = router
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        body_of(response).await
    }

    async fn post_choice(
        router: &axum::Router,
        uri: &str,
        body: &'static str,
    ) -> axum::response::Response {
        router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", form_type())
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    fn progress_counts(state: &AppState, profile_id: i64, lesson_id: u32) -> (i64, i64) {
        let db = state.db();
        crate::store::progress_for_profile(&db, profile_id)
            .expect("read progress")
            .into_iter()
            .find(|row| row.lesson_id == lesson_id)
            .map(|row| (row.correct_count, row.wrong_count))
            .unwrap_or((0, 0))
    }

    #[tokio::test]
    async fn answer_redirects_and_get_shows_flash() {
        let state = AppState::in_memory().unwrap();
        let router = add_an(app(state)).await;

        let right = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(right.status(), StatusCode::SEE_OTHER);
        let right_at = right.headers()["location"].to_str().unwrap().to_string();
        assert_eq!(right_at, "/profiles/1/bai/1?kq=dung&tiep=2");

        let shown = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&right_at)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(shown.status(), StatusCode::OK);
        let html = body_of(shown).await;
        assert!(html.contains("Giỏi quá, đúng rồi"));
        assert!(html.contains("/profiles/1/bai/2"));

        // Lesson 2's correct choice is index 1, so 0 is a miss.
        let wrong = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/2")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(wrong.status(), StatusCode::SEE_OTHER);
        let wrong_at = wrong.headers()["location"].to_str().unwrap().to_string();
        assert_eq!(wrong_at, "/profiles/1/bai/2?kq=sai");

        let retry = router
            .oneshot(
                Request::builder()
                    .uri(&wrong_at)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(retry.status(), StatusCode::OK);
        let html = body_of(retry).await;
        assert!(html.contains("Chưa đúng. Thử lại nhé"));
    }

    #[tokio::test]
    async fn refresh_after_redirect_does_not_double_correct_count() {
        let state = AppState::in_memory().unwrap();
        let router = add_an(app(state.clone())).await;

        let posted = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles/1/bai/1")
                    .header("content-type", form_type())
                    .body(Body::from("choice=0"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(posted.status(), StatusCode::SEE_OTHER);
        let location = posted.headers()["location"].to_str().unwrap().to_string();

        // The browser follows the 303, then F5 repeats that GET. Neither grades again.
        for _ in 0..2 {
            let response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&location)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let html = body_of(response).await;
            assert!(html.contains("Giỏi quá, đúng rồi"));
        }

        assert_eq!(progress_counts(&state, 1, 1), (1, 0));
        let home = router
            .oneshot(
                Request::builder()
                    .uri("/profiles/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(home).await;
        assert!(html.contains("⭐ 1 sao"));
    }

    #[tokio::test]
    async fn correct_banner_requires_a_finished_lesson() {
        let router = add_an(test_app()).await;

        let forged = get_html(&router, "/profiles/1/bai/1?kq=dung").await;
        assert!(!forged.contains("Giỏi quá"));
        assert!(!forged.contains("confetti-anchor"));

        // Lesson 3's correct choice is index 2, so 0 leaves done = 0.
        let missed = post_choice(&router, "/profiles/1/bai/3", "choice=0").await;
        assert_eq!(missed.status(), StatusCode::SEE_OTHER);
        let still_open = get_html(&router, "/profiles/1/bai/3?kq=dung").await;
        assert!(!still_open.contains("Giỏi quá"));
        assert!(!still_open.contains("confetti-anchor"));
        let retry = get_html(&router, "/profiles/1/bai/3?kq=sai").await;
        assert!(retry.contains("Chưa đúng. Thử lại nhé"));

        let posted = post_choice(&router, "/profiles/1/bai/1", "choice=0").await;
        assert_eq!(posted.status(), StatusCode::SEE_OTHER);
        let shown = get_html(&router, "/profiles/1/bai/1?kq=dung").await;
        assert!(shown.contains("Giỏi quá"));
        assert!(shown.contains("confetti-anchor"));
    }

    #[tokio::test]
    async fn forged_tiep_points_at_a_real_lesson_in_the_subject() {
        let router = add_an(test_app()).await;
        let posted = post_choice(&router, "/profiles/1/bai/1", "choice=0").await;
        assert_eq!(posted.status(), StatusCode::SEE_OTHER);

        // 99999 does not exist. 55 is a real lesson in another subject.
        for tiep in ["99999", "55"] {
            let html = get_html(&router, &format!("/profiles/1/bai/1?kq=dung&tiep={tiep}")).await;
            assert!(
                html.contains("/profiles/1/bai/2"),
                "tiep={tiep} should fall back to the next real lesson"
            );
            assert!(
                !html.contains(&format!("/profiles/1/bai/{tiep}")),
                "tiep={tiep} leaked into the page"
            );
        }

        let kept = get_html(&router, "/profiles/1/bai/1?kq=dung&tiep=3").await;
        assert!(kept.contains("/profiles/1/bai/3"));
    }

    #[test]
    fn next_unfinished_wraps_to_earlier_gap_then_none() {
        let lessons = for_subject(Subject::Toan);
        let mut done: HashSet<u32> = lessons.iter().map(|lesson| lesson.id).collect();
        done.remove(&1);
        assert_eq!(next_unfinished_id(&lessons, 3, &done), Some(1));
        done.insert(1);
        assert_eq!(next_unfinished_id(&lessons, 3, &done), None);
    }

    #[tokio::test]
    async fn broken_tiep_link_still_renders_the_lesson() {
        let router = add_an(test_app()).await;
        let posted = post_choice(&router, "/profiles/1/bai/1", "choice=0").await;
        assert_eq!(posted.status(), StatusCode::SEE_OTHER);

        for link in [
            "/profiles/1/bai/1?kq=dung&tiep=abc",
            "/profiles/1/bai/1?kq=dung&tiep=",
            "/profiles/1/bai/1?kq=dung&tiep=-1",
        ] {
            let html = get_html(&router, link).await;
            assert!(html.contains("Giỏi quá"), "{link}");
            assert!(html.contains("/profiles/1/bai/2"), "{link}");
        }
    }

    #[tokio::test]
    async fn poisoned_db_lock_does_not_take_down_later_requests() {
        let state = AppState::in_memory().unwrap();
        let router = add_an(app(state.clone())).await;

        // Poison the mutex the way a panic inside a handler would.
        let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = state.db();
            panic!("poison the db lock");
        }));
        assert!(poisoned.is_err());

        let html = get_html(&router, "/profiles").await;
        assert!(html.contains("Ai đang học?"));
    }

    #[tokio::test]
    async fn answer_is_refused_for_a_profile_of_another_parent() {
        let state = AppState::in_memory().unwrap();
        let router = add_an(app(state.clone())).await;
        {
            let db = state.db();
            let other = upsert_user(&db, "sub-other", "other@example.com").unwrap();
            let theirs = add_profile(&db, other.id, "Binh", "cat").unwrap();
            assert_eq!(theirs.id, 2);
        }

        let response = post_choice(&router, "/profiles/2/bai/1", "choice=0").await;
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Không có hồ sơ này"));
        assert!(!html.contains("Giỏi quá"));

        let db = state.db();
        assert!(crate::store::progress_for_profile(&db, 2)
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn answer_write_failure_shows_an_error_instead_of_a_star() {
        let state = AppState::in_memory().unwrap();
        let router = add_an(app(state.clone())).await;
        {
            let db = state.db();
            db.execute_batch("DROP TABLE answer_events;").unwrap();
        }

        let response = post_choice(&router, "/profiles/1/bai/1", "choice=0").await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let html = body_of(response).await;
        assert!(html.contains("Không lưu được kết quả"));
        assert!(!html.contains("Giỏi quá"));
        assert!(!html.contains("+1 sao"));
    }
}
