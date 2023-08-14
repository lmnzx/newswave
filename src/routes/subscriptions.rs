use axum::{http::StatusCode, response::IntoResponse, Extension, Form};
use redis::AsyncCommands;
use std::sync::Arc;
use validator::Validate;

use crate::email_service::send_email;
use crate::AppState;

#[derive(serde::Deserialize, Validate)]
pub struct FormData {
    #[validate(email)]
    email: String,
    #[validate(length(min = 3))]
    name: String,
}

pub async fn subscribe(
    Extension(app_state): Extension<Arc<AppState>>,
    Form(input): Form<FormData>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    let redis_client = app_state.redis_client.clone();

    let mut con = redis_client.get_async_connection().await.unwrap();

    match input.validate() {
        Ok(_) => (),
        Err(e) => {
            tracing::info!("failed validation: {:?}", e);
            return (StatusCode::BAD_REQUEST, "Bad Request: invalid data").into_response();
        }
    }

    tracing::info!("subscription request from: {email}", email = input.email);

    match sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'pending_confirmation')",
        uuid::Uuid::new_v4(),
        input.email,
        input.name,
        chrono::Utc::now()
    ).execute(pool).await {
        Ok(_) => {
            tracing::info!("new subscriber added to db");

            let token = uuid::Uuid::new_v4().to_string();

           con.set_ex::<String, String, ()>(token.to_owned(), input.email.to_owned(), 60 * 30).await.unwrap();

            let body = format!("Welcome to NewsWave, please confirm your email by clicking on the link below: \n\n http://localhost:3000/subscribe/{}\n\n The link is only valid for 30mins.", token);

            send_email(input.email, "Welcome to NewsWave".to_owned(), body).await;

            (StatusCode::OK, "Congratulations you have subscribers to NewsWave").into_response()
        },
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error: failed to execute query").into_response()
        }
    }
}
