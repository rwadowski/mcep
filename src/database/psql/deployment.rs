use crate::database::DeploymentStore;
use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::Sink;
use crate::types::deployment::source::Source;
use crate::types::deployment::{
    DeployedBlock, Deployment, DeploymentId, NewDeployment, UpdateDeployment,
};
use crate::utils;
use log::{error, info};
use sqlx::types::Json;
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct PsqlDeploymentStore {
    pool: Pool<Postgres>,
}

impl Clone for PsqlDeploymentStore {
    fn clone(&self) -> Self {
        PsqlDeploymentStore {
            pool: self.pool.clone(),
        }
    }
}

impl PsqlDeploymentStore {
    pub fn new(pool: Pool<Postgres>) -> PsqlDeploymentStore {
        PsqlDeploymentStore { pool }
    }
}

impl DeploymentStore for PsqlDeploymentStore {
    async fn create_deployment(&self, new_deployment: &NewDeployment) -> Result<Deployment, String> {
        let mut builder = insert_deployment_builder(new_deployment);
        let query = builder.build_query_as::<Deployment>();
        let result = query.fetch_one(&self.pool).await;
        match result {
            Ok(deployment) => {
                info!("deployment {} created", deployment.id);
                Ok(deployment)
            }
            Err(err) => {
                error!("deployment creation failed - {}", err);
                Err(err.to_string())
            }
        }
    }

    async fn update_deployment(&self, update: &UpdateDeployment) -> Result<Deployment, String> {
        let mut builder = update_deployment_builder(update);
        let query = builder.build_query_as::<Deployment>();
        let result = query.fetch_one(&self.pool).await;
        match result {
            Ok(deployment) => {
                info!("deployment {} updated", deployment.id);
                Ok(deployment)
            }
            Err(err) => {
                error!("deployment {} update failed - {}", update.id, err);
                Err(err.to_string())
            }
        }
    }

    async fn get_deployment(&self, id: DeploymentId) -> Result<Deployment, String> {
        let result = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await;
        result.map_err(utils::log_and_convert_to_string)
    }

    async fn get_all_deployments(&self) -> Result<Vec<Deployment>, String> {
        let result = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments")
            .fetch_all(&self.pool)
            .await;
        result.map_err(utils::log_and_convert_to_string)
    }

    async fn delete_deployment(&self, id: DeploymentId) -> Result<(), String> {
        let mut builder = delete_deployment_builder(id);
        let query = builder.build();
        let result = query.fetch_one(&self.pool).await;
        result.map(|_| ()).map_err(utils::log_and_convert_to_string)
    }
}

fn insert_deployment_builder(new_deployment: &NewDeployment) -> QueryBuilder<Postgres> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO deployments (name, version, connections, sources, sinks, blocks)",
    );
    query_builder.push_values([new_deployment], |mut b, dep| {
        b.push_bind(dep.name.clone())
            .push_bind(dep.version.clone())
            .push_bind(Json::<Vec<BlockConnection>>(dep.connections.clone()))
            .push_bind(Json::<Vec<Source>>(dep.sources.clone()))
            .push_bind(Json::<Vec<Sink>>(dep.sinks.clone()))
            .push_bind(Json::<Vec<DeployedBlock>>(dep.blocks.clone()));
    });
    query_builder.push(" RETURNING *");
    query_builder
}

fn update_deployment_builder(deployment: &UpdateDeployment) -> QueryBuilder<Postgres> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE deployments SET ");
    let mut sep = query_builder.separated(", ");
    if let Some(name) = &deployment.name {
        sep.push("name = ").push_bind_unseparated(name.clone());
    }
    if let Some(version) = &deployment.version {
        sep.push("version = ")
            .push_bind_unseparated(version.clone());
    }
    if let Some(connections) = &deployment.connections {
        sep.push("connections = ")
            .push_bind_unseparated(Json::<Vec<BlockConnection>>::from(connections.clone()));
    }
    if let Some(sources) = &deployment.sources {
        sep.push("sources = ")
            .push_bind_unseparated(Json::<Vec<Source>>::from(sources.clone()));
    }
    if let Some(sinks) = &deployment.sinks {
        sep.push("sinks = ")
            .push_bind_unseparated(Json::<Vec<Sink>>::from(sinks.clone()));
    }
    if let Some(blocks) = &deployment.blocks {
        sep.push("blocks = ")
            .push_bind_unseparated(Json::<Vec<DeployedBlock>>::from(blocks.clone()));
    }
    query_builder.push(" WHERE id = ");
    query_builder.push_bind(deployment.id);
    query_builder.push(" RETURNING *");
    query_builder
}

fn delete_deployment_builder(id: DeploymentId) -> QueryBuilder<Postgres> {
    let mut builder = QueryBuilder::new("DELETE FROM deployments WHERE id = ");
    builder.push_bind(id);
    builder.push(" RETURNING id");
    builder
}
