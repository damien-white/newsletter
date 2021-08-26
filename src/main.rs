//! `main.rs` Serves as the entrypoint to the application.
//!
//! The application is bootstrapped and launched via the [`start`] function.
//!
//! [`start`]: newsletter::server::start

use std::net::TcpListener;

use newsletter::server::start;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    let addr = listener.local_addr()?;
    println!("Starting service. Running at: http://{}", addr);

    start(listener)?.await
}
