//! tests/health.rs

// Launch application in the background using `tokio::spawn`
fn start_server() {
    let server = newsletter::run().expect("Server address binding failed");
    let _ = tokio::spawn(server);
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
    start_server();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health")
        .send()
        .await
        .expect("No response. HTTP request failed.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
