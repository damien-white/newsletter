use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

/// The `subscribe` endpoint handles POST requests generated from submitted
/// HTML forms containing user data.
pub async fn subscribe(_form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
