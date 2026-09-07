pub mod html;
pub mod lessons;
pub mod store;

use crate::html::Flash;
use crate::lessons::{by_id, for_subject, next_after, Subject};
use crate::store::{
    add_profile, list_profiles, upsert_user, Profile, StoreError, MAX_PROFILES_PER_USER,
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
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/profiles", get(profiles_page).post(create_profile))
        .route("/profiles/:id", get(open_profile))
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
        Some(profile) => Html(html::home(&profile)),
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
    Html(html::subject_page(&profile, subject, &for_subject(subject)))
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
    let flash = if form.choice == lesson.correct {
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
    Html(html::lesson_page(
        &profile,
        lesson,
        index,
        lessons.len(),
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
        assert!(html.contains("Bắt đầu bài 1"));
        assert!(html.contains("54 bài"));
        assert!(html.contains("Các số từ 0 đến 10"));
        assert!(html.contains("Cộng trừ trong phạm vi 10"));
        assert!(html.contains("Các số đến 100"));
        assert!(html.contains("Bài 1. Số 0 đến 5"));
        assert!(html.contains("/profiles/1/bai/1"));
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
