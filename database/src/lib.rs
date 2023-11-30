use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn init_connection_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Unable to connect to postgres")
}

pub async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!("../database/migrations").run(pool).await
}
