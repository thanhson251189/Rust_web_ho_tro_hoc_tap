use web_ho_tro_hoc_tap::{app, AppState};

#[tokio::main]
async fn main() {
    let state = AppState::in_memory().expect("open store");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind 127.0.0.1:3000");
    axum::serve(listener, app(state)).await.expect("serve");
}
