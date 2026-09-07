pub mod store;

use crate::store::{add_profile, list_profiles, upsert_user, StoreError, MAX_PROFILES_PER_USER};
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

/// Grade-1 KNTT subject outline shown on the profile hub (hardcoded this slice).
struct Subject {
    slug: &'static str,
    name: &'static str,
    topics: &'static [&'static str],
}

const SUBJECTS: &[Subject] = &[
    Subject {
        slug: "toan",
        name: "Toán",
        topics: &["Số đếm", "Phép cộng", "Hình học"],
    },
    Subject {
        slug: "tieng-viet",
        name: "Tiếng Việt",
        topics: &["Chữ cái", "Âm vần", "Từ vựng"],
    },
    Subject {
        slug: "tieng-anh",
        name: "Tiếng Anh",
        topics: &["Greetings", "Numbers", "Colors"],
    },
];

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
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/profiles", get(profiles_page).post(create_profile))
        .route("/profiles/:id", get(open_profile))
        .route("/subjects/:slug", get(subject_stub))
        .with_state(state)
}

async fn home() -> Redirect {
    Redirect::to("/profiles")
}

async fn profiles_page(State(state): State<AppState>) -> Html<String> {
    let db = state.db.lock().expect("db lock");
    let profiles = list_profiles(&db, state.user_id).expect("list profiles");
    Html(render_picker(&profiles, None))
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
        Some("Nhap ten ho so.".to_string())
    } else {
        match add_profile(&db, state.user_id, &name, &avatar_key) {
            Ok(_) => None,
            Err(StoreError::ProfileLimit) => Some("Toi da 2 ho so.".to_string()),
            Err(err) => Some(format!("Khong luu duoc ho so: {err:?}")),
        }
    };
    let profiles = list_profiles(&db, state.user_id).expect("list profiles");
    Html(render_picker(&profiles, error.as_deref()))
}

async fn open_profile(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = state.db.lock().expect("db lock");
    let profiles = list_profiles(&db, state.user_id).expect("list profiles");
    match profiles.into_iter().find(|p| p.id == id) {
        Some(profile) => Html(render_hub(&profile)),
        None => Html(
            "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8></head><body><p>Khong co ho so nay.</p><p><a href=/profiles>Quay lai</a></p></body></html>".into(),
        ),
    }
}

async fn subject_stub(Path(slug): Path<String>) -> Html<String> {
    let subject = SUBJECTS.iter().find(|s| s.slug == slug);
    let title = subject.map(|s| s.name).unwrap_or("Môn học");
    Html(format!(
        "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8><meta name=viewport content='width=device-width, initial-scale=1'><title>{0}</title><style>body{{font-family:sans-serif;margin:2rem;}} a{{color:inherit;}}</style></head><body><h1>{0}</h1><p>Đang xây dựng bài luyện KNTT...</p><p><a href=/profiles>Quay lại</a></p></body></html>",
        escape(title)
    ))
}

fn shared_style() -> &'static str {
    "body{font-family:sans-serif;margin:2rem;} .row{display:flex;gap:1rem;flex-wrap:wrap;} .card{display:flex;flex-direction:column;align-items:center;width:8rem;padding:1.5rem;border:2px solid #333;border-radius:1rem;text-decoration:none;color:inherit;font-size:1.4rem;} .avatar{font-size:2rem;} .name{text-align:center;} .topics{font-size:0.85rem;margin-top:0.75rem;color:#444;text-align:left;width:100%;padding:0;list-style:disc inside;} .error{color:#b00020;} form{margin-top:2rem;display:flex;gap:1rem;align-items:end;flex-wrap:wrap;} .header{display:flex;align-items:center;gap:1rem;margin-bottom:1.5rem;} .header .avatar{font-size:2rem;} .header a{margin-left:auto;}"
}

fn render_hub(profile: &store::Profile) -> String {
    let mut cards = String::new();
    for subject in SUBJECTS {
        let mut topics = String::new();
        for topic in subject.topics {
            topics.push_str(&format!("<li>{}</li>", escape(topic)));
        }
        cards.push_str(&format!(
            "<a class=card href=/subjects/{0}><span class=name>{1}</span><ul class=topics>{2}</ul></a>",
            escape(subject.slug),
            escape(subject.name),
            topics
        ));
    }

    format!(
        "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8><meta name=viewport content='width=device-width, initial-scale=1'><title>Chon mon</title><style>{style}</style></head><body><div class=header><span class=avatar>{avatar}</span><span class=name>{name}</span><a href=/profiles>Doi ho so</a></div><h1>Chon mon hoc</h1><div class=row>{cards}</div></body></html>",
        style = shared_style(),
        avatar = escape(&profile.avatar_key),
        name = escape(&profile.name),
        cards = cards
    )
}

fn render_picker(profiles: &[store::Profile], error: Option<&str>) -> String {
    let mut cards = String::new();
    for p in profiles {
        cards.push_str(&format!(
            "<a class=card href=/profiles/{0}><span class=avatar>{1}</span><span class=name>{2}</span></a>",
            p.id,
            escape(&p.avatar_key),
            escape(&p.name)
        ));
    }

    let form = if profiles.len() < MAX_PROFILES_PER_USER {
        "<form method=post action=/profiles><label>Ten <input name=name required maxlength=24></label><label>Avatar <select name=avatar_key><option value=robot>robot</option><option value=cat>cat</option><option value=bear>bear</option><option value=fox>fox</option></select></label><button type=submit>Them ho so</button></form>".to_string()
    } else {
        String::new()
    };

    let err = error
        .map(|e| format!("<p class=error>{}</p>", escape(e)))
        .unwrap_or_default();

    format!(
        "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8><meta name=viewport content='width=device-width, initial-scale=1'><title>Chon ho so</title><style>{style}</style></head><body><h1>Ai dang hoc?</h1><div class=row>{cards}</div>{err}{form}</body></html>",
        style = shared_style(),
        cards = cards,
        err = err,
        form = form
    )
}

fn escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => {
                out.push('&');
                out.push_str("amp;");
            }
            '<' => {
                out.push('&');
                out.push_str("lt;");
            }
            '>' => {
                out.push('&');
                out.push_str("gt;");
            }
            '"' => {
                out.push('&');
                out.push_str("quot;");
            }
            _ => out.push(c),
        }
    }
    out
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
        assert!(html.contains("Ai dang hoc?"));
        assert!(html.contains("Them ho so"));
    }

    #[tokio::test]
    async fn create_two_profiles_then_reject_third() {
        let app = test_app();
        let form_type = concat!("application/x-www-form-", "urlencoded");

        let first = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type)
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
                    .header("content-type", form_type)
                    .body(Body::from("name=Binh&avatar_key=cat"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(second).await;
        assert!(html.contains("An"));
        assert!(html.contains("Binh"));
        assert!(!html.contains("Them ho so"));

        let third = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_type)
                    .body(Body::from("name=Chi&avatar_key=bear"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let html = body_of(third).await;
        assert!(html.contains("Toi da 2 ho so."));
        assert!(!html.contains("Chi"));
    }

    fn form_content_type() -> &'static str {
        concat!("application/x-www-form-", "urlencoded")
    }

    fn first_profile_id(html: &str) -> i64 {
        let marker = "href=/profiles/";
        let start = html
            .find(marker)
            .map(|i| i + marker.len())
            .expect("profile link");
        let digits: String = html[start..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        digits.parse().expect("profile id")
    }

    async fn post_profile(app: axum::Router, body: &'static str) -> String {
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/profiles")
                    .header("content-type", form_content_type())
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        body_of(response).await
    }

    #[tokio::test]
    async fn open_profile_shows_subject_hub_not_xin_chao() {
        let app = test_app();
        let picker = post_profile(app.clone(), "name=An&avatar_key=robot").await;
        let id = first_profile_id(&picker);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/profiles/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;

        assert!(html.contains("An"));
        assert!(html.contains("Toán"));
        assert!(html.contains("Tiếng Việt"));
        assert!(html.contains("Tiếng Anh"));
        assert!(html.contains("Số đếm"));
        assert!(html.contains("Phép cộng"));
        assert!(html.contains("Hình học"));
        assert!(html.contains("Chữ cái"));
        assert!(html.contains("Âm vần"));
        assert!(html.contains("Từ vựng"));
        assert!(html.contains("Greetings"));
        assert!(html.contains("Numbers"));
        assert!(html.contains("Colors"));
        assert!(html.contains("href=/subjects/toan"));
        assert!(html.contains("href=/subjects/tieng-viet"));
        assert!(html.contains("href=/subjects/tieng-anh"));
        assert!(!html.contains("Xin chao"));
    }

    #[tokio::test]
    async fn open_profile_unknown_id_keeps_message() {
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
        assert!(html.contains("Khong co ho so nay"));
        assert!(!html.contains("Toán"));
    }

    #[tokio::test]
    async fn subject_stub_returns_placeholder() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/subjects/toan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = body_of(response).await;
        assert!(html.contains("Toán"));
        assert!(html.contains("Đang xây dựng bài luyện KNTT"));
        assert!(html.contains("href=/profiles"));
    }
}
