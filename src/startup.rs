use crate::configuration::get_configuration;
use crate::routes::app;

pub async fn run() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}
