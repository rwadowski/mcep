use crate::types::config::Database as DatabaseConfig;
use crate::types::definition::{Definition, UpdateDefinition};
use crate::types::deployment::Deployment;
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_connection_pool(cfg: &DatabaseConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(cfg.url().as_str())
        .await
        .expect("Unable to connect to postgres")
}

pub async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    let migrator = sqlx::migrate!("./migrations");
    migrator.run(pool).await
}

pub trait Database {
    async fn create_definition(); //??
    async fn delete_definition(); //??
    async fn get_definition(id: i64) -> Result<Definition, String>; // ?
    async fn update_definition(update: UpdateDefinition) -> Result<Definition, String>; //?

    async fn create_deployment(); //??

    async fn update_deployment(); //??

    async fn delete_deployment(); //??
    async fn get_deployment(ids: Vec<i64>) -> Result<Vec<Deployment>, MigrateError>; // ??
}
