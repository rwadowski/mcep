use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Type};

pub mod block;
pub mod error;
pub mod mod_test;

pub type DefinitionId = i32;

#[derive(Serialize, Deserialize, Eq, PartialEq, FromRow, Debug)]
pub struct Definition {
    pub id: DefinitionId,
    pub name: String,
    pub version: String,
    pub body: Value,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "snake_case", type_name = "VARCHAR")]
pub enum DataType {
    Boolean,
    UnsignedInt,
    SignedInt,
    FloatType,
    Text,
    // Array(Box<DataType>),
    // Map(Box<DataType>, Box<DataType>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DefinitionField {
    pub value: String,
}

impl From<&str> for DefinitionField {
    fn from(value: &str) -> Self {
        DefinitionField {
            value: value.to_string(),
        }
    }
}
