use crossbeam_channel::Sender;
use rocket::log::private::{error, info};
use rocket::serde::Deserialize;
use types::deployment::{Command, Deployment};
use sqlx::{Pool, Postgres};
use sqlx::types::Json;
use types::definition::{Definition, DefinitionId};
use types::deployment::Command::Deploy;
use types::deployment::connection::BlockConnection;
use types::deployment::sink::Sink;
use types::deployment::source::Source;
use crate::definition::get::get_definitions;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub connections: Vec<BlockConnection>,
    pub source: Vec<Source>,
    pub sink: Vec<Sink>
}

impl NewDeployment {
    pub fn definition_ids(&self) -> Vec<DefinitionId> {
        let mut result: Vec<DefinitionId> = Vec::new();
        for connection in self.connections.iter() {
            if let Some(id) = connection.from.definition_id_opt() {
                result.push(id);
            }
            if let Some(id) = connection.to.definition_id_opt() {
                result.push(id);
            }
        }
        result
    }
}

pub async fn create_deployment(sender: &Sender<Command>, pool: &Pool<Postgres>, new_deployment: NewDeployment) -> Option<Deployment> {
    let definition_ids: Vec<DefinitionId> = new_deployment.definition_ids();
    let write_result: Result<Deployment, String> = sqlx::query_as::<_, Deployment>("INSERT INTO deployment (name, version, application_id, connections, source, sink) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *")
        .bind(new_deployment.name)
        .bind(new_deployment.version)
        .bind(Json::<Vec<BlockConnection>>(new_deployment.connections))
        .bind(Json::<Vec<Source>>(new_deployment.source))
        .bind(Json::<Vec<Sink>>(new_deployment.sink))
        .fetch_one(pool)
        .await
        .map_err(|err| err.to_string());
    let definitions = get_definitions(pool, definition_ids).await;
    let tuple = to_tuple(write_result, definitions);
    let result = tuple
        .and_then(|(deployment, definitions)| {
            sender.send(Deploy(deployment.clone(), definitions))
                .map_err(|err| err.to_string())
                .map(|_| deployment)
        });
    match result {
        Ok(d) => {
            info!("deployment {} created", d.id.to_string());
            Some(d)
        }
        Err(err) => {
            error!("{}", err.to_string());
            None
        },
    }
}

fn to_tuple(deployment_opt: Result<Deployment, String>, definitions_opt: Result<Vec<Definition>, String>) -> Result<(Deployment, Vec<Definition>), String> {
    deployment_opt.and_then(|deployment| {
        definitions_opt.and_then(|definitions|{
            Ok((deployment, definitions))
        })
    })
}