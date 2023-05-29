use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use sqlx::{Pool, Postgres};

pub async fn init_connection_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await.expect("Unable to connect to postgres")
}
