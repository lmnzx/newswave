use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

async fn handler() -> Html<&'static str> {
    info!("handling request...");
    Html("<p>Hello, World!</p>")
}

async fn handler_404() -> impl IntoResponse {
    info!("handling 404...");
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    info!("starting server...");

    let app = Router::new().route("/", get(handler)).fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
