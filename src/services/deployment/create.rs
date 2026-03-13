use std::collections::HashSet;

use log::{error, info};
use serde_derive::Deserialize;
use sqlx::types::Json;
use sqlx::{Pool, Postgres};

use crate::runtime::engine::Engine;
use crate::services::definition::get::get_definitions;
use crate::types::definition::DefinitionId;
use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::Sink;
use crate::types::deployment::source::Source;
use crate::types::deployment::{DeployedBlock, Deployment};
use crate::utils;

#[derive(Deserialize)]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub connections: Vec<BlockConnection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
    pub blocks: Vec<DeployedBlock>,
}

pub async fn create_deployment(
    engine: &Engine,
    pool: &Pool<Postgres>,
    new_deployment: NewDeployment,
) -> Option<Deployment> {
    let deployment: Deployment = sqlx::query_as::<_, Deployment>("INSERT INTO deployments (name, version, connections, sources, sinks, blocks) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(new_deployment.name)
        .bind(new_deployment.version)
        .bind(Json::<Vec<BlockConnection>>(new_deployment.connections))
        .bind(Json::<Vec<Source>>(new_deployment.sources))
        .bind(Json::<Vec<Sink>>(new_deployment.sinks))
        .bind(Json::<Vec<DeployedBlock>>(new_deployment.blocks))
        .fetch_one(pool)
        .await
        .map_err(utils::log_and_convert_to_string)
        .ok()?;

    let definition_ids: HashSet<DefinitionId> = deployment.definition_ids();
    let definitions = get_definitions(pool, definition_ids)
        .await
        .map_err(utils::log_and_convert_to_string)
        .ok()?;

    info!("deployment {} created", deployment.id);

    let definitions_map: std::collections::HashMap<_, _> =
        definitions.into_iter().map(|d| (d.id, d)).collect();
    match engine.deploy(&deployment, &definitions_map).await {
        Ok(()) => {
            info!("deployment {} deployed", deployment.id);
            Some(deployment)
        }
        Err(err) => {
            error!("deployment {} not deployed: {}", deployment.id, err);
            None
        }
    }
}
