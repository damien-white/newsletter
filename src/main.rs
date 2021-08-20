//! `main.rs` Serves as the entrypoint to the application.
//!
//! The [`run`] function is executed and awaited, starting the service.
//!
//! [`run`]: newsletter::run

use newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
