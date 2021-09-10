use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscribeForm {
    name: String,
    email: String,
}

/// The `subscribe` endpoint handles POST requests generated from submitted
/// HTML forms containing user data.
pub async fn subscribe(form: web::Form<SubscribeForm>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // `Spans`, like logs, have an associated `level`
    let request_span = tracing::info_span!(
        "Creating subscriber.",
        %request_id,
        subscriber_name = %form.name,
        subscriber_email = %form.email
    );

    // Using `enter` in an async function is not a good idea.
    // FIXME: remove afterwards; DO NOT LET INTO PROD BUILD
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span! macro. `.instrument` takes
    // care of it a the right moments in the query future lifetime.
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match query!(
        "INSERT INTO subscriptions (id, name, email, created_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
