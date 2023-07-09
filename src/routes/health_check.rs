use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    tracing::info!("health check endpoint hit");

    (StatusCode::OK, "everything is fine, boss 👍")
}
