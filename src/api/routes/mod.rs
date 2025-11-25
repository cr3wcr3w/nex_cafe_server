//! API route definitions

use axum::Router;
use sqlx::SqlitePool;

// Import route modules
pub mod auth;
mod health;

use self::auth::login;

/// Combine all routes into a single router with the database pool
pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .merge(health::router())
        .merge(login::router())
        .with_state(pool)
}
