use serde::{Serialize, Deserialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use crate::definition::connection::Connection;
use crate::definition::connection::sink::Sink;
use crate::definition::connection::source::Source;
use crate::definition::Id;

#[derive(Debug)]
pub enum Command {
    Deploy(Deployment),
    Undeploy(DeploymentId)
}

pub type DeploymentId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub id: DeploymentId,
    pub name: String,
    pub version: String,
    pub connections: Vec<Connection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
}

impl FromRow<'_, PgRow> for Deployment {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: DeploymentId = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let version: String = row.try_get("version")?;
        let connections_js: Json<Vec<Connection>> = row.try_get("connections")?;
        let connections = connections_js.0;
        let sources_js: Json<Vec<Source>> = row.try_get("sources")?;
        let sources = sources_js.0;
        let sinks_js: Json<Vec<Sink>> = row.try_get("sinks")?;
        let sinks = sinks_js.0;
        Ok(
            Deployment {
                id, name, version, connections, sources, sinks,
            }
        )
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct BlockId {
    pub value: String
}

impl BlockId {
    pub fn new(deployment_id: &DeploymentId, id: &Id) -> BlockId {
        BlockId {
            value: deployment_id.to_string() + "." + id.value.as_str(),
        }
    }
}
