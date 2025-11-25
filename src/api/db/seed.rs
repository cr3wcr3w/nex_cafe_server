use crate::api::utils::hash;
use sqlx::{sqlite::SqlitePool, Error as SqlxError};
use tracing::info;

pub async fn init(pool: &SqlitePool) -> Result<(), SqlxError> {
    // users
    let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    if users_count == 0 {
        sqlx::query("INSERT INTO users (email, password_hash, role_id) VALUES (?, ?, 1)")
            .bind("root@nexcafe.local")
            .bind(hash::hash_password("toor").expect("Failed to hash password"))
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO users (email, password_hash, role_id) VALUES (?, ?, 2)")
            .bind("customer@nexcafe.local")
            .bind(hash::hash_password("remotsuc").expect("Failed to hash password"))
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO users (email, password_hash, role_id) VALUES (?, ?, 3)")
            .bind("staff@nexcafe.local")
            .bind(hash::hash_password("ffats").expect("Failed to hash password"))
            .execute(pool)
            .await?;

        info!("Demo user: root@nexcafe.local/toor, staff@nexcafe.local/ffats, customer@nexcafe.local/remotsuc");
    }

    // roles
    let roles_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM roles")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if roles_count == 0 {
        sqlx::query(
            r#"
            INSERT INTO roles (name) VALUES ('admin'), ('customer'), ('staff')
            "#,
        )
        .execute(pool)
        .await?;
        info!("Added default roles");
    }

    // statuses
    let statuses_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM statuses")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if statuses_count == 0 {
        sqlx::query(
            r#"
            INSERT INTO statuses (name, color) VALUES 
            ('online', 'yellow'),
            ('offline', 'orange'),
            ('in_use', 'green'),
            ('error', 'red')
            "#,
        )
        .execute(pool)
        .await?;
        info!("Added default statuses");
    }

    Ok(())
}
