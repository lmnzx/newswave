use std::sync::Arc;

use axum::extract::Json;
use axum::{response::IntoResponse, Extension, Json as ResponseJson};
use serde::Deserialize;

use crate::email_service::send_email;
use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct Letter {
    subject: String,
    body: String,
}

#[derive(Debug)]
struct Subscribers {
    email: String,
}

pub async fn publish_newsletter(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<Letter>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    tracing::info!("received a new newsletter");

    let subject = &payload.subject;
    let body = &payload.body;

    match sqlx::query!(
        "INSERT INTO publish (id, letter, published_at) VALUES ($1, $2, $3)",
        uuid::Uuid::new_v4(),
        payload.body,
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            tracing::info!("saved the newsletter");
            let subs = sqlx::query_as!(
                Subscribers,
                "SELECT email FROM subscriptions WHERE status = 'confirmed'"
            )
            .fetch_all(pool)
            .await
            .unwrap();

            for x in subs {
                send_email(x.email, subject.to_string(), body.to_string()).await;
            }

            ResponseJson("got it")
        }
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);

            ResponseJson("failed to save it")
        }
    }
}
