pub mod store;

use crate::store::{
    add_profile, list_profiles, upsert_user, StoreError, MAX_PROFILES_PER_USER,
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

const LOCAL_PARENT_SUB: &str = "local-dev";
const AVATARS: &[&str] = &["robot", "cat", "bear", "fox"];

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Connection>>,
    user_id: i64,
}

impl AppState {
    pub fn in_memory() -> Result<Self, StoreError> {
        let conn = store::open_memory()?;
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
        Some(profile) => Html(format!(
            "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8><title>{0}</title></head><body><p>Xin chao, {0}</p><p><a href=/profiles>Doi ho so</a></p></body></html>",
            escape(&profile.name)
        )),
        None => Html(
            "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8></head><body><p>Khong co ho so nay.</p><p><a href=/profiles>Quay lai</a></p></body></html>".into(),
        ),
    }
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
        "<!DOCTYPE html><html lang=vi><head><meta charset=utf-8><meta name=viewport content='width=device-width, initial-scale=1'><title>Chon ho so</title><style>body{{font-family:sans-serif;margin:2rem;}} .row{{display:flex;gap:1rem;flex-wrap:wrap;}} .card{{display:flex;flex-direction:column;align-items:center;width:8rem;padding:1.5rem;border:2px solid #333;border-radius:1rem;text-decoration:none;color:inherit;font-size:1.4rem;}} .avatar{{font-size:2rem;}} .error{{color:#b00020;}} form{{margin-top:2rem;display:flex;gap:1rem;align-items:end;flex-wrap:wrap;}}</style></head><body><h1>Ai dang hoc?</h1><div class=row>{cards}</div>{err}{form}</body></html>"
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
}
