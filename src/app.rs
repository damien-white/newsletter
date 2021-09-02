//! The server module bootstraps the application and starts the service
//! using the user-supplied configuration settings.

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

use crate::handlers::{health, subscribe};

pub fn start(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/health", web::get().to(health))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
