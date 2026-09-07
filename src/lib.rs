pub mod html;
pub mod lessons;
pub mod store;

use crate::html::Flash;
use crate::lessons::{by_id, for_subject, next_after, Lesson, Subject};
use crate::store::{
    add_profile, list_profiles, next_lesson_id, record_answer, upsert_user, Profile, StoreError,
    MAX_PROFILES_PER_USER,
};
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing::get,
    Form, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

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

    fn profiles(&self) -> Vec<Profile> {
        let db = self.db.lock().expect("db lock");
        list_profiles(&db, self.user_id).expect("list profiles")
    }

    fn profile(&self, id: i64) -> Option<Profile> {
        self.profiles().into_iter().find(|profile| profile.id == id)
    }

    fn done_lesson_ids(&self, profile_id: i64) -> std::collections::HashSet<u32> {
        let db = self.db.lock().expect("db lock");
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

    let db = state.db.lock().expect("db lock");
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
                let db = state.db.lock().expect("db lock");
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
        let db = state.db.lock().expect("db lock");
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
    let db = state.db.lock().expect("db lock");
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

async fn lesson_page(
    State(state): State<AppState>,
    Path((id, lesson_id)): Path<(i64, u32)>,
) -> Html<String> {
    render_lesson(&state, id, lesson_id, None)
}

#[derive(Deserialize)]
struct Answer {
    choice: usize,
}

async fn answer_lesson(
    State(state): State<AppState>,
    Path((id, lesson_id)): Path<(i64, u32)>,
    Form(form): Form<Answer>,
) -> Html<String> {
    let Some(lesson) = by_id(lesson_id) else {
        return Html(html::missing());
    };
    let correct = form.choice == lesson.correct;
    {
        let db = state.db.lock().expect("db lock");
        let _ = record_answer(&db, id, lesson_id, correct);
    }
    let flash = if correct {
        Flash::Correct {
            next_id: next_after(lesson).map(|next| next.id),
        }
    } else {
        Flash::Wrong
    };
    render_lesson(&state, id, lesson_id, Some(flash))
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
        let db = state.db.lock().expect("db lock");
        crate::store::progress_for_profile(&db, profile_id)
            .expect("read progress")
            .iter()
            .map(|p| p.correct_count)
            .sum::<i64>()
            .max(0) as usize
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
    use super::{app, AppState};
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
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
}
