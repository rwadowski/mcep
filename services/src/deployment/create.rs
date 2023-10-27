use crossbeam_channel::Sender;
use rocket::log::private::{error, info};
use rocket::serde::Deserialize;
use types::deployment::{Command, Deployment};
use sqlx::{Error, Pool, Postgres};
use types::definition::{Definition, DefinitionId};
use types::deployment::Command::Deploy;
use types::deployment::connection::DefinitionConnection;
use crate::definition::get::get_definitions;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub application_id: i32,
    pub connections: Vec<DefinitionConnection>,
    pub source: String,
    pub sink: String
}

impl NewDeployment {
    pub fn definition_ids(&self) -> Vec<DefinitionId> {
        let mut result: Vec<DefinitionId> = Vec::new();
        for connection in self.connections.iter() {
            result.push( connection.from.block);
            result.push(connection.to.block);
        }
        result
    }
}

pub async fn create_deployment(sender: &Sender<Command>, pool: &Pool<Postgres>, new_deployment: NewDeployment) -> Option<Deployment> {
    if let Ok(raw_conn) = raw_connections(&new_deployment) {
        let definition_ids: Vec<DefinitionId> = new_deployment.definition_ids();
        let write_result: Result<Deployment, String> = sqlx::query_as::<_, Deployment>("INSERT INTO deployment (name, version, application_id, connections, source, sink) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *")
            .bind(new_deployment.name)
            .bind(new_deployment.version)
            .bind(new_deployment.application_id)
            .bind(raw_conn)
            .bind(new_deployment.source)
            .bind(new_deployment.sink)
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
    } else {
        None
    }
}

fn raw_connections(new_deployment: &NewDeployment) -> Result<String, String> {
    let connections: String = serde_json::to_string(&new_deployment.connections)
        .map_err(|err| err.to_string())?;
    Ok(connections)
}

fn to_tuple(deployment_opt: Result<Deployment, String>, definitions_opt: Result<Vec<Definition>, String>) -> Result<(Deployment, Vec<Definition>), String> {
    deployment_opt.and_then(|deployment| {
        definitions_opt.and_then(|definitions|{
            Ok((deployment, definitions))
        })
    })
}