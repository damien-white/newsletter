//! `main.rs` Serves as the entrypoint to the application.
//!
//! The [`run`] function is executed and awaited, starting the service.
//!
//! [`run`]: newsletter::run

use std::net::TcpListener;

use newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
