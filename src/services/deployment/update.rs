use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::Sink;
use crate::types::deployment::source::Source;
use crate::types::deployment::{DeployedBlock, Deployment, UpdateDeployment};
use log::{error, info};
use sqlx::types::Json;
use sqlx::{Pool, Postgres, QueryBuilder};

pub async fn update_deployment<'a>(
    pool: &Pool<Postgres>,
    update: UpdateDeployment,
) -> Result<Deployment, String> {
    let mut builder = new_builder(&update);
    let query = builder.build_query_as::<Deployment>();
    let result = query.fetch_one(pool).await;
    match result {
        Ok(deployment) => {
            info!("deployment {} updated", deployment.id);
            Ok(deployment)
        }
        Err(err) => {
            error!("deployment {} update failed: {}", update.id, err);
            Err(err.to_string())
        }
    }
}

pub(crate) fn new_builder(deployment: &UpdateDeployment) -> QueryBuilder<Postgres> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE deployments SET ");
    let mut sep = query_builder.separated(", ");
    let mut has_fields = false;
    if let Some(name) = &deployment.name {
        sep.push("name = ").push_bind_unseparated(name.clone());
        has_fields = true;
    }
    if let Some(version) = &deployment.version {
        sep.push("version = ")
            .push_bind_unseparated(version.clone());
        has_fields = true;
    }
    if let Some(connections) = &deployment.connections {
        sep.push("connections = ")
            .push_bind_unseparated(Json::<Vec<BlockConnection>>::from(connections.clone()));
        has_fields = true;
    }
    if let Some(sources) = &deployment.sources {
        sep.push("sources = ")
            .push_bind_unseparated(Json::<Vec<Source>>::from(sources.clone()));
        has_fields = true;
    }
    if let Some(sinks) = &deployment.sinks {
        sep.push("sinks = ")
            .push_bind_unseparated(Json::<Vec<Sink>>::from(sinks.clone()));
        has_fields = true;
    }
    if let Some(blocks) = &deployment.blocks {
        sep.push("blocks = ")
            .push_bind_unseparated(Json::<Vec<DeployedBlock>>::from(blocks.clone()));
        has_fields = true;
    }
    query_builder.push(if has_fields { " WHERE id = " } else { "WHERE id = " });
    query_builder.push_bind(deployment.id);
    query_builder.push(" RETURNING *");
    query_builder
}
