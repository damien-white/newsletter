use actix_web::HttpResponse;

/// The `health_check` endpoint is used for testing, inspection and monitoring.
/// It returns a "200 OK" response to indicate that the service is running.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
