use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

use newswave::routes::{global_404, health_check, subscribe};

/*
    TODO:
    - [x] add new subscriber
    - [ ] verify new subscriber
*/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    tracing::info!("starting server...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/newswave")
        .await
        .unwrap();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .with_state(pool)
        .fallback(global_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
