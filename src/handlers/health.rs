use actix_web::HttpResponse;

/// The `health` endpoint is useful for testing, inspection and monitoring.
/// It returns a "200 OK" response to indicate that the service is running.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
