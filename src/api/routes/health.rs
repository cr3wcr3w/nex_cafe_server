use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use dirs;
use serde::Serialize;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tracing::warn;

use crate::{api::constant::http_status_phrases as message, shared::constants::PROJECTNAME};

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
    message: &'static str,
}

#[derive(Serialize)]
struct HealthData {
    version: &'static str,
    database: DatabaseStatus,
}

#[derive(Serialize)]
struct DatabaseStatus {
    exists: bool,
    path: String,
}

fn get_database_path() -> PathBuf {
    dirs::data_local_dir()
        .or_else(|| dirs::data_dir())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"))
        .join(PROJECTNAME)
        .join("app.db")
}

async fn health_check() -> impl IntoResponse {
    let db_path = get_database_path();
    let db_exists = db_path.exists();

    let (status, message) = if db_exists {
        (StatusCode::OK, message::OK)
    } else {
        warn!("Database file not found at: {}", db_path.display());
        (
            StatusCode::SERVICE_UNAVAILABLE,
            message::SERVICE_UNAVAILABLE,
        )
    };

    let response = ApiResponse {
        status: status.as_str().to_owned(),
        message,
        data: HealthData {
            version: env!("CARGO_PKG_VERSION"),
            database: DatabaseStatus {
                exists: db_exists,
                path: db_path.to_string_lossy().into_owned(),
            },
        },
    };

    (status, Json(response))
}

pub fn router() -> Router<SqlitePool> {
    Router::new().route("/health", get(health_check))
}
