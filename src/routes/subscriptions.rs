use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use sqlx::PgPool;
use validator::Validate;

/*
    TODO:
    - [x] get form data from request
    - [x] insert new subscriber into db
*/

#[derive(serde::Deserialize, Validate)]
pub struct FormData {
    #[validate(email)]
    email: String,
    #[validate(length(min = 3))]
    name: String,
}

pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(input): Form<FormData>,
) -> impl IntoResponse {
    match input.validate() {
        Ok(_) => (),
        Err(e) => {
            tracing::info!("failed validation: {:?}", e);
            return (StatusCode::BAD_REQUEST, "Bad Request: invalid data").into_response();
        }
    }

    tracing::info!("subscription request from: {email}", email = input.email);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        uuid::Uuid::new_v4(),
        input.email,
        input.name,
        chrono::Utc::now()
    ).execute(&pool).await {
        Ok(_) => {
            tracing::info!("new subscriber added to db");
            (StatusCode::OK, "Congratulations you have subscribers to NewsWave").into_response()
        },
        Err(e) => {
            tracing::error!("failed to execute query: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error: failed to execute query").into_response()
        }
    }
}
