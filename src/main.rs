use web_ho_tro_hoc_tap::{app, listen_addr, AppState, DEFAULT_DB_PATH};

#[tokio::main]
async fn main() {
    let state = AppState::from_file(DEFAULT_DB_PATH).expect("open store");
    let addr = listen_addr(std::env::var("HOTRO_ADDR").ok().as_deref());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|err| panic!("bind {addr}: {err}"));
    axum::serve(listener, app(state)).await.expect("serve");
}
