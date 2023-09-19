use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};

use newswave::config::Settings;
use newswave::routes::{global_404, health_check, subscribe, subscriptions_confirm};
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

    let app_state = Arc::new(AppState {
        pool,
        redis_client,
    });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .route("/subscribe/:token", get(subscriptions_confirm))
        .layer(Extension(app_state.clone()))
        .fallback(global_404);

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
