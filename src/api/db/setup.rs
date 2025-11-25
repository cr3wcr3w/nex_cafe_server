use crate::api::db::seed;
use crate::api::migration;
use crate::shared::constants::PROJECTNAME;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::fs;
use tracing::{error, info};

pub async fn init_sqlite() -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    // create a path for the SQLite database file
    let db_path = dirs::data_local_dir()
        .or_else(|| dirs::data_dir())
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"))
        .join(PROJECTNAME);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&db_path).map_err(|e| {
        error!(
            "Failed to create database directory at {}: {}",
            db_path.display(),
            e
        );
        e
    })?;

    let db_file = db_path.join("app.db");
    info!("Using database at: {}", db_file.display());

    // Connect to SQLite database file with create_if_missing
    let conn_opts = SqliteConnectOptions::new()
        .filename(&db_file)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(conn_opts)
        .await
        .map_err(|e| {
            error!(
                "Failed to connect to database at {}: {}",
                db_file.display(),
                e
            );
            e
        })?;

    // ensure foreign keys are enforced
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    // migrations
    migration::run(&pool).await?;

    // seed initial data
    seed::init(&pool).await?;

    Ok(pool)
}
