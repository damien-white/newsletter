use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

/// The `subscribe` endpoint handles POST requests generated from submitted
/// HTML forms containing user data.
pub async fn subscribe(form: web::Form<SubscribeForm>, pool: web::Data<PgPool>) -> HttpResponse {
    match query!(
        "INSERT INTO subscriptions (id, name, email, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
