use axum::{routing::get, Router};
use std::net::SocketAddr;

use newswave::routes::{global_404, health_check};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    tracing::info!("starting server...");

    let app = Router::new()
        .route("/health_check", get(health_check))
        .fallback(global_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
