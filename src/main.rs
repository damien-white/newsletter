//! `main.rs` Serves as the entrypoint to the application.
//!
//! The application is bootstrapped and launched via the [`start`] function.
//!
//! [`start`]: newsletter::server::start

use std::net::TcpListener;

use sqlx::PgPool;

use newsletter::{
    app::start,
    settings::Settings,
    telemetry::{init_subscriber, register_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = register_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    // Load settings from configuration source(s). Panic on failure.
    let settings = Settings::load().expect("Failed to load configuration settings.");

    let pool =
        PgPool::connect_lazy(&settings.database.url()).expect("Failed to connect to PostgreSQL");
    let address = &format!("{}:{}", settings.app.host, settings.app.port);
    let listener = TcpListener::bind(address)?;

    start(listener, pool)?.await
}
