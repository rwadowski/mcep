use crossbeam_channel::Sender;
use rocket::log::private::{error, info};
use rocket::serde::Deserialize;
use types::deployment::{Command, Deployment};
use sqlx::{Error, Pool, Postgres};
use types::definition::connection::Connection;
use types::deployment::Command::Deploy;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub application_id: i32,
    pub connections: Vec<Connection>,
    pub source: String,
    pub sink: String
}
pub async fn create_deployment(sender: &Sender<Command>, pool: &Pool<Postgres>, new_deployment: NewDeployment) -> Option<Deployment> {
    if let Ok(raw_conn) = raw_connections(&new_deployment) {
        let write_result: Result<Deployment, Error> = sqlx::query_as::<_, Deployment>("INSERT INTO deployment (name, version, application_id, connections, source, sink) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *")
            .bind(new_deployment.name)
            .bind(new_deployment.version)
            .bind(new_deployment.application_id)
            .bind(raw_conn)
            .bind(new_deployment.source)
            .bind(new_deployment.sink)
            .fetch_one(pool)
            .await;
        let result = write_result
            .map_err(|err| err.to_string())
            .and_then(|depl| {
                sender.send(Deploy(depl.clone()))
                    .map_err(|err| err.to_string())
                    .map(|_| depl)
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