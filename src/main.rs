use web_ho_tro_hoc_tap::{app, AppState, DEFAULT_DB_PATH};

#[tokio::main]
async fn main() {
    let state = AppState::from_file(DEFAULT_DB_PATH).expect("open store");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind 127.0.0.1:3000");
    axum::serve(listener, app(state)).await.expect("serve");
}
