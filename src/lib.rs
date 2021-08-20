//! lib.rs Holds the core logic of the application.

use actix_web::{web, App, HttpResponse, HttpServer};

pub async fn run() -> std::io::Result<()> {
    println!("Starting service. Available at: http://127.0.0.1:8000");

    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

/// The `health` endpoint is useful for testing, inspection and monitoring.
/// It returns a "200 OK" response to indicate that the service is running.
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
