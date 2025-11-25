use sqlx::SqlitePool;

mod m2025_11_20;

// Run all raw-SQL migrations in order
pub async fn run(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    m2025_11_20::up(pool).await?;
    Ok(())
}
