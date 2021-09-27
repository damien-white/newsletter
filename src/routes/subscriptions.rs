use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing::instrument;
use uuid::Uuid;

// TODO: Separate the data access layer from the route handler; add new module

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

/// The `subscribe` endpoint handles POST requests generated from submitted
/// HTML forms containing user data.
#[allow(clippy::async_yields_async)] // See: https://github.com/tokio-rs/tracing/issues/1450
#[instrument(
    name = "Adding new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<SubscribeForm>, pool: web::Data<PgPool>) -> HttpResponse {
    match create_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[instrument(name = "Saving subscriber record to the database", skip(form, pool))]
pub async fn create_subscriber(pool: &PgPool, form: &SubscribeForm) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO subscriptions (id, name, email, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
    })?;

    Ok(())
}
