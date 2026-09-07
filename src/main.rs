use web_ho_tro_hoc_tap::app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind 127.0.0.1:3000");
    axum::serve(listener, app()).await.expect("serve");
}
