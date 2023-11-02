use axum::response::IntoResponse;
use tracing::info;

pub async fn login(multipart: Multipart) -> impl IntoResponse {
    info!("hii from login")
}
