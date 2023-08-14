use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use redis::AsyncCommands;

use crate::AppState;

pub async fn subscriptions_confirm(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    tracing::info!("subscriptions_confirm endpoint hit");

    let pool = &app_state.pool;

    let redis_client = app_state.redis_client.clone();

    let mut con = redis_client.get_async_connection().await.unwrap();

    let email: Option<String> = con.get(&token).await.unwrap();

    match email {
        Some(email) => {
            sqlx::query!(
                "UPDATE subscriptions SET status = 'confirmed' WHERE email = $1",
                email
            )
            .execute(pool)
            .await
            .unwrap();

            con.del::<_, i32>(&token).await.unwrap();

            return (StatusCode::OK, "Subscription confirmed");
        }
        None => return (StatusCode::NOT_FOUND, "Token not found"),
    }
}
