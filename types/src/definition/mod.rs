pub mod block;
pub mod error;
pub mod mod_test;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use sqlx::FromRow;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Id {
    pub value: String
}

impl Id {
    pub fn new(id: &str) -> Id {
        Id {
            value: id.to_string()
        }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<'d, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        let str: String = String::deserialize(deserializer)?;
        Ok(Id::new(str.as_str()))
    }
}

pub type DefinitionId = i32;

#[derive(Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, FromRow)]
pub struct Definition {
    pub id: DefinitionId,
    pub name: String,
    pub version: String,
    pub body: String,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    UnsignedInt,
    SignedInt,
    FloatType,
    Text,
    Array(Box<DataType>),
    Map(Box<DataType>, Box<DataType>),
}

