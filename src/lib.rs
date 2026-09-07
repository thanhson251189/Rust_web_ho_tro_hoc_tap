pub mod store;

use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(home))
}

async fn home() -> &'static str {
    "Xin chào"
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn home_returns_greeting() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(bytes.as_ref(), "Xin chào".as_bytes());
    }
}
