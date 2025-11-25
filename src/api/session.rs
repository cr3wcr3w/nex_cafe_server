use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use sqlx::SqlitePool;

pub async fn setup_session(pool: SqlitePool) -> SessionLayer<SessionSqlitePool> {
    // Create a new SQLite pool specifically for sessions
    let session_pool = SessionSqlitePool::from(pool);

    // Create session configuration
    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_secure(false); // Set to true in production with HTTPS

    // Create the session store
    let session_store = SessionStore::<SessionSqlitePool>::new(Some(session_pool), session_config)
        .await
        .expect("Failed to create session store");

    // Create and return the session layer
    SessionLayer::new(session_store)
}
