/// Integration tests for the `/health` endpoint
use std::net::TcpListener;

use sqlx::{query, Connection, PgConnection};

use newsletter::settings::Settings;

// Launch application in the background using `tokio::spawn`
fn spawn_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Server failed to bind to random port");

    // Get the port that the spawned server is bound to
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::app::start(listener).expect("Server address binding failed");
    let _ = tokio::spawn(server);

    // Return the application address
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_returns_correct_response() {
    // Arrange
    let address = spawn_server();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("HTTP client request failed");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[actix_rt::test]
async fn subscribe_returns_success_if_form_valid() {
    // Arrange
    let address = spawn_server();
    let config = Settings::load().expect("Failed to load configuration settings.");
    let database_url = config.database.build_url();
    let mut connection = PgConnection::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL.");

    let client = reqwest::Client::new();
    let body = "name=Peter%20Donovan&email=peter.donovan@gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("HTTP client request failed");

    // Assert
    assert_eq!(response.status().as_u16(), 200);

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.name, "Peter Donovan");
    assert_eq!(saved.email, "peter.donovan@gmail.com");
}

#[actix_rt::test]
async fn subscribe_returns_bad_request_if_form_invalid() {
    // Arrange
    let address = spawn_server();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Peter%20Donovan", "missing email field"),
        ("email=peter.donovan@gmail.com", "missing name field"),
        ("", "missing name and email fields"),
    ];

    // Act
    for (body, err_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("HTTP client request failed");

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The server did not respond with a '400 Bad Request' when {}",
            err_message
        );
    }
}
