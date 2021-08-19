use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service. Available at: http://127.0.0.1:8000");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}
