pub mod psql;

use crate::types::config::Database as DatabaseConfig;
use crate::types::definition::{Definition, DefinitionId, NewDefinition, UpdateDefinition};
use crate::types::deployment::{Deployment, DeploymentId, NewDeployment, UpdateDeployment};
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

pub trait DefinitionStore: Send + Sync {
    async fn create_definition(&self, definition: &NewDefinition) -> Result<Definition, String>;

    async fn update_definition(&self, update: &UpdateDefinition) -> Result<Definition, String>;

    async fn get_definition(&self, id: DefinitionId) -> Result<Definition, String>;

    async fn get_definitions(&self, ids: &Vec<DefinitionId>) -> Result<Vec<Definition>, String>;

    async fn get_all_definitions(&self) -> Result<Vec<Definition>, String>;

    async fn delete_definition(&self, id: DefinitionId) -> Result<(), String>;
}

pub trait DeploymentStore: Send + Sync {
    async fn create_deployment(&self, new_deployment: &NewDeployment) -> Result<Deployment, String>;

    async fn update_deployment(&self, update: &UpdateDeployment) -> Result<Deployment, String>;

    async fn get_deployment(&self, id: DeploymentId) -> Result<Deployment, String>;

    async fn get_all_deployments(&self) -> Result<Vec<Deployment>, String>;

    async fn delete_deployment(&self, id: DeploymentId) -> Result<(), String>;
}
