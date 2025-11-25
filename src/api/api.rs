use axum::{serve, Router};
use tokio::runtime::Builder;
use tracing::info;

use crate::api::db::setup::init_sqlite;
use crate::api::routes::router;
use crate::api::session::setup_session;
use sqlx::SqlitePool;

pub fn start_server() {
    std::thread::spawn(|| {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            // Initialize persistent SQLite database and run migrations/seed
            let db: SqlitePool = init_sqlite().await.expect("init sqlite");

            // Set up session layer
            let session_layer = setup_session(db.clone()).await;

            // Create the router with database pool
            let api = router(db);

            // Create the app with session layer and routes
            let app = Router::new().nest("/api", api).layer(session_layer);

            // Bind to all interfaces so clients on the LAN can reach this host
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

            info!("Axum server listening on:");
            info!("- local:  http://127.0.0.1:3000");

            serve(listener, app).await.unwrap();
        });
    });
}
