use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

pub async fn init_db(database_url: &str) -> crate::Result<SqlitePool> {
    info!("Connecting to database: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database initialized successfully");
    Ok(pool)
}
