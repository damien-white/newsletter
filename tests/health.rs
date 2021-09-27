/// Integration tests for the `/health` endpoint
use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{migrate, query, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use newsletter::app::start;
use newsletter::settings::{DatabaseSettings, Settings};
use newsletter::telemetry::{init_subscriber, register_subscriber};

// Initialize the `tracing` stack once via `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test";
    let filter_level = "info";
    // FIXME: Cleanup code; consider replacing `tracing` / `telemetry` layer
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = register_subscriber(subscriber_name, filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = register_subscriber(subscriber_name, filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestServer {
    pub address: String,
    pub pool: PgPool,
}

/// Spawn a new task instance of the server on a random port and run it in the background
async fn spawn_server() -> TestServer {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Server failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut settings = Settings::load().expect("Failed to load configuration settings.");
    settings.database.dbname = Uuid::new_v4().to_string();
    let pool = setup_test_db(&settings.database).await;

    let server = start(listener, pool.clone()).expect("Server address binding failed");
    tokio::spawn(server);

    TestServer { address, pool }
}

pub async fn setup_test_db(database: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&database.test_url())
        .await
        .expect("Failed to connect to PostgreSQL.");

    connection
        .execute(&*format!("CREATE DATABASE \"{}\";", database.dbname))
        .await
        .expect("Failed to create database instance.");

    // Run database migrations
    let pool = PgPool::connect(&database.url())
        .await
        .expect("Failed to connect to PostgreSQL");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Database migrations failed!");

    pool
}

#[actix_rt::test]
async fn health_check_returns_correct_response() {
    // Arrange
    let server = spawn_server().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &server.address))
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
    let server = spawn_server().await;
    let pool = server.pool.clone();
    let client = reqwest::Client::new();
    let body = "name=Peter%20Donovan&email=peter.donovan@gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscribe", &server.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("HTTP client request failed");

    // Assert
    assert_eq!(response.status().as_u16(), 201);

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.name, "Peter Donovan");
    assert_eq!(saved.email, "peter.donovan@gmail.com");
}

#[actix_rt::test]
async fn subscribe_returns_bad_request_if_form_invalid() {
    // Arrange
    let server = spawn_server().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Peter%20Donovan", "missing email field"),
        ("email=peter.donovan@gmail.com", "missing name field"),
        ("", "missing name and email fields"),
    ];

    // Act
    for (body, err_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &server.address))
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
