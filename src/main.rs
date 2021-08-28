//! `main.rs` Serves as the entrypoint to the application.
//!
//! The application is bootstrapped and launched via the [`start`] function.
//!
//! [`start`]: newsletter::server::start

use std::net::TcpListener;

use newsletter::app::start;
use newsletter::settings::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load settings from configuration source(s). Panic on failure.
    let settings = Settings::load().expect("Failed to load settings from config source.");

    let addr = &format!("127.0.0.1:{}", settings.app.port);
    let listener = TcpListener::bind(addr)?;

    start(listener)?.await
}
