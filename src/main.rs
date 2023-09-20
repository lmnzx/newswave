use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::{ServeDir, ServeFile};

use newswave::config::Settings;
use newswave::routes::{health_check, subscribe, subscriptions_confirm};
use newswave::AppState;

/*
    TODO
    [ ] publish newsletter
    [ ] send newsletter
   ![ ] error handling
*/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let s = Settings::get_config().expect("Failed to load configuration");

    tracing::info!("starting server...");

    tracing::info!("{:?}", s);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(s.database.connection_string());

    let redis_client = Arc::new(redis::Client::open(s.redis.connection_string()).unwrap());

    let app_state = Arc::new(AppState { pool, redis_client });

    let app = Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/subscribe", post(subscribe))
        .route("/api/subscribe/:token", get(subscriptions_confirm))
        .layer(Extension(app_state.clone()))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("./web/assets")).handle_error(|e| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", e),
                )
            }),
        )
        .fallback_service(
            get_service(ServeFile::new("./web/index.html")).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        );

    // let addr = SocketAddr::from(([127, 0, 0, 1], s.application.port));
    let addr = match (s.application.host.parse(), s.application.port) {
        (Ok(ip), port) => SocketAddr::new(ip, port),
        _ => panic!("Invalid address"),
    };

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
