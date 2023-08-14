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
    [x] send confirmation email
    [ ] publish newsletter
    [ ] send newsletter
    [-] configuration // almost done still needs refactoring
   ![ ] error handling
    [ ] deploy
*/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let s = Settings::get_config().expect("Failed to load configuration");

    tracing::info!("starting server...");

    // connecting to postgres
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost:5432/newswave")
    //     .await
    //     .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy_with(s.database.connection_string());

    // connecting to redis
    // let redis_client = Arc::new(redis::Client::open("redis://localhost:6379").unwrap());

    let redis_client = Arc::new(redis::Client::open(s.redis.connection_string()).unwrap());

    let app_state = Arc::new(AppState {
        pool: pool.clone(),
        redis_client: redis_client,
    });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .route("/subscribe/:token", get(subscriptions_confirm))
        .layer(Extension(app_state.clone()))
        .fallback(global_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], s.application.port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
