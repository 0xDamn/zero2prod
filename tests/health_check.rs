//! tests/health_check.rs

use sqlx::{Connection, PgConnection};
use tokio::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::routes::app;

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app().await;

    let client = reqwest::Client::builder().no_proxy().build().unwrap();

    let resp = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .unwrap();

    // Assert the response
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    // Here you can parse the response body if needed
    // And perform further assertions
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    // Now you can send a request to the server

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    //Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

/// Spin up an instance of our application
/// and returns its address (i.e. http://localhost:XXXX)
async fn spawn_app() -> String {
    // Bind to a dynamic port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Start the server in the background
    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });

    format!("http://{}", addr)
}
