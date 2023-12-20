use actix::Addr;
use rocket::log::private::{error, info};
use rocket::serde::Deserialize;
use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::collections::HashSet;

use crate::runtime::engine::{EngineActor, EngineActorMessage, EngineActorResponse};
use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::Sink;
use crate::types::deployment::source::Source;
use crate::types::deployment::{DeployedBlock, Deployment};

use crate::services::definition::get::get_definitions;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub connections: Vec<BlockConnection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
    pub blocks: Vec<DeployedBlock>,
}

impl NewDeployment {
    pub fn definition_ids(&self) -> HashSet<DefinitionId> {
        HashSet::from_iter(self.blocks.iter().map(|block| block.definition_id))
    }
}

pub async fn create_deployment(
    sender: &Addr<EngineActor>,
    pool: &Pool<Postgres>,
    new_deployment: NewDeployment,
) -> Option<Deployment> {
    let definition_ids: HashSet<DefinitionId> = new_deployment.definition_ids();
    let write_result: Result<Deployment, String> = sqlx::query_as::<_, Deployment>("INSERT INTO deployments (name, version, connections, sources, sinks, blocks) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(new_deployment.name)
        .bind(new_deployment.version)
        .bind(Json::<Vec<BlockConnection>>(new_deployment.connections))
        .bind(Json::<Vec<Source>>(new_deployment.sources))
        .bind(Json::<Vec<Sink>>(new_deployment.sinks))
        .bind(Json::<Vec<DeployedBlock>>(new_deployment.blocks))
        .fetch_one(pool)
        .await
        .map_err(|err| err.to_string());
    let definitions = get_definitions(pool, definition_ids).await;
    let result = to_tuple(write_result, definitions);
    match result {
        Ok((deployment, definitions)) => {
            info!("deployment {} created", deployment.id.to_string());
            let actor_result = sender
                .send(EngineActorMessage::Deploy(deployment.clone(), definitions))
                .await;
            actor_result.ok().and_then(|response| match response {
                EngineActorResponse::Succeed => {
                    info!("deployment {} deployed", deployment.id.to_string());
                    Some(deployment)
                }
                EngineActorResponse::Failed(err) => {
                    error!(
                        "deployment {} not deployed due to {}",
                        deployment.id.to_string(),
                        err
                    );
                    None
                }
            })
        }
        Err(err) => {
            error!("{}", err.to_string());
            None
        }
    }
}

fn to_tuple(
    deployment_opt: Result<Deployment, String>,
    definitions_opt: Result<Vec<Definition>, String>,
) -> Result<(Deployment, Vec<Definition>), String> {
    deployment_opt.and_then(|deployment| {
        definitions_opt.and_then(|definitions| Ok((deployment, definitions)))
    })
}