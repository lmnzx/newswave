use std::sync::Arc;

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
    pub redis_client: Arc<redis::Client>,
}
