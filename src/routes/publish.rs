use axum::{http::StatusCode,  response::IntoResponse};
use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Letter {
    body: String,
}

pub async fn publish_newsletter(Json(payload): Json<Letter>) -> impl IntoResponse {
    tracing::info!("send newsletter");
    tracing::info!("{:?}", payload.body);

    (StatusCode::OK, "everything is fine, boss 👍")
}
