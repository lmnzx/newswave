mod global_404;
mod health_check;
mod subscriptions;
mod subscriptions_confirm;
mod publish;

use axum::{
    routing::{get, post},
    Router,
};
pub use global_404::*;
pub use health_check::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
pub use publish::*;

pub fn routes() -> Router {
    Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/subscribe", post(subscribe))
        .route("/api/subscribe/:token", get(subscriptions_confirm))
        .route("/api/publish", post(publish_newsletter))
}
