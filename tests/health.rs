//! tests/health.rs

use std::net::TcpListener;

// Launch application in the background using `tokio::spawn`
fn spawn_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Server failed to bind to random port");

    // Get the port that the spawned server is bound to
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Server address binding failed");
    let _ = tokio::spawn(server);

    // Return the application address
    format!("http://127.0.0.1:{}", port)
}

/// Note: `actix_rt::test` is the testing equivalent of `actix_web::main`
/// It spares one from having to specify the `#[test]` attribute.
/// We add `actix-rs` and `reqwest` under `[dev-dependencies]` in order to
/// be able to conduct black-box testing.
///
/// NOTE: the code generated can be inspected using
/// `cargo expand --test health`
/// We are using the `arrange — act — assert` testing pattern.
///
/// For a more detailed explanation of the pattern, please refer to this blog
/// article on [Automation Panda](https://automationpanda.com/2020/07/07/arrange-act-assert-a-pattern-for-writing-good-tests/)
#[actix_rt::test]
async fn health_endpoint_returns_correct_response() {
    // Arrange
    let address = spawn_server();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("HTTP request failed; no response from server");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
