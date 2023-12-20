use std::fmt;
use std::fmt::Formatter;

use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_str;
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{Error, FromRow, Row};

use crate::types::definition::DefinitionId;
use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::Sink;
use crate::types::deployment::source::Source;

pub mod connection;
mod mod_test;
pub mod sink;
mod sink_test;
pub mod source;
mod source_test;

pub type DeploymentId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub id: DeploymentId,
    pub name: String,
    pub version: String,
    pub connections: Vec<BlockConnection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
    pub blocks: Vec<DeployedBlock>,
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
        let blocks_js: Json<Vec<DeployedBlock>> = row.try_get("blocks")?;
        let blocks = blocks_js.0;
        Ok(Deployment {
            id,
            name,
            version,
            connections,
            sources,
            sinks,
            blocks,
        })
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub struct BlockId {
    pub definition_id: DefinitionId,
    pub id: BlockInstanceId,
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "BlockId({}.{})", self.definition_id, self.id)
    }
}

impl TryFrom<&str> for BlockId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let elements: Vec<&str> = value.split(".").collect();
        if elements.len() == 2 {
            Ok(BlockId {
                definition_id: from_str::<DefinitionId>(elements[0])
                    .map_err(|err| err.to_string())?,
                id: from_str::<i32>(elements[1]).map_err(|err| err.to_string())?,
            })
        } else {
            Err(format!(
                "block id '{}' must contain two elements delimited by '.'",
                value
            ))
        }
    }
}

pub type BlockInstanceId = i32;

impl BlockId {
    pub fn new(definition_id: DefinitionId, id: BlockInstanceId) -> BlockId {
        BlockId { definition_id, id }
    }

    pub fn to_string(self) -> String {
        format!("{}.{}", self.definition_id, self.id)
    }
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = format!("{}.{}", self.definition_id, self.id);
        serializer.serialize_str(str.as_str())
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

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub struct BlockInput {
    value: String,
}

impl Serialize for BlockInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for BlockInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        Ok(BlockInput { value })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DeployedBlock {
    pub definition_id: DefinitionId,
    pub id: BlockInstanceId,
}

impl DeployedBlock {
    pub fn new(definition_id: DefinitionId, id: BlockInstanceId) -> DeployedBlock {
        DeployedBlock { definition_id, id }
    }
    pub fn id(&self) -> BlockId {
        BlockId::new(self.definition_id, self.id)
    }
}
