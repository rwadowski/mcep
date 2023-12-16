use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use types::config::Database as DatabaseConfig;

pub async fn init_connection_pool(cfg: &DatabaseConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(cfg.url().as_str())
        .await
        .expect("Unable to connect to postgres")
}

pub async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!("../database/migrations").run(pool).await
}
