use crate::definition::{Definition, Id};
use crate::deployment::connection::BlockConnection;
use crate::deployment::sink::Sink;
use crate::deployment::source::Source;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{Error, FromRow, Row};

pub mod connection;
mod mod_test;
pub mod sink;
mod sink_test;
pub mod source;
mod source_test;

#[derive(Debug)]
pub enum Command {
    Deploy(Deployment, Vec<Definition>),
    Undeploy(DeploymentId),
}

pub type DeploymentId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub id: DeploymentId,
    pub name: String,
    pub version: String,
    pub connections: Vec<BlockConnection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
}

impl Deployment {
    pub fn block_id(&self) -> BlockId {
        let id = Id::new(self.name.as_str());
        BlockId::new(&self.id, &id)
    }
}

impl FromRow<'_, PgRow> for Deployment {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        let id: DeploymentId = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let version: String = row.try_get("version")?;
        let connections_js: Json<Vec<BlockConnection>> = row.try_get("connections")?;
        let connections = connections_js.0;
        let sources_js: Json<Vec<Source>> = row.try_get("sources")?;
        let sources = sources_js.0;
        let sinks_js: Json<Vec<Sink>> = row.try_get("sinks")?;
        let sinks = sinks_js.0;
        Ok(Deployment {
            id,
            name,
            version,
            connections,
            sources,
            sinks,
        })
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub struct BlockId {
    pub value: String,
}

impl TryFrom<&str> for BlockId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let elements: Vec<&str> = value.split(".").collect();
        if elements.len() == 2 {
            Ok(BlockId {
                value: value.to_string(),
            })
        } else {
            Err(format!(
                "block id '{}' must contain two elements delimited by '.'",
                value
            ))
        }
    }
}

impl BlockId {
    pub fn new(deployment_id: &DeploymentId, id: &Id) -> BlockId {
        BlockId {
            value: deployment_id.to_string() + "." + id.value.as_str(),
        }
    }
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for BlockId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str: String = String::deserialize(deserializer)?;
        BlockId::try_from(str.as_str()).map_err(|err| D::Error::custom(err))
    }
}
