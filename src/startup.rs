use crate::routes::app;

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app()).await.unwrap();
}
