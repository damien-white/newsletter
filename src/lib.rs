//! lib.rs Holds the core logic of the application.

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!(
        "Starting service. Available at: http://{}",
        listener.local_addr()?
    );

    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

/// The `health` endpoint is useful for testing, inspection and monitoring.
/// It returns a "200 OK" response to indicate that the service is running.
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// The `subscribe` endpoint handles POST requests generated from submitted
/// HTML forms containing user data.
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
