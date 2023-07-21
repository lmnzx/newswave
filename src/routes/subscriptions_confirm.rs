use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

pub async fn subscriptions_confirm(
    // State(pool): State<PgPool>,
    // State(redis_client): State<Arc<redis::aio::Connection>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    tracing::info!("health check endpoint hit");

    (StatusCode::OK, "everything is fine, boss 👍")
}
