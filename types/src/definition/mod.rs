pub mod block;
pub mod connection;
pub mod error;
pub mod mod_test;

use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use sqlx::FromRow;
use crate::definition::block::Block;
use crate::definition::connection::Connection;
use crate::definition::connection::sink::Sink;
use crate::definition::connection::source::Source;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id(pub String);

impl Id {
    pub fn new(id: &str) -> Id {
        Id(id.to_string())
    }
}

#[derive(Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, FromRow)]
pub struct Definition {
    pub id: i32,
    pub title: String,
    pub version: String,
    pub body: String,
    pub body_type: String,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    UnsignedInt,
    SignedInt,
    FloatType,
    Text,
    Array(Box<DataType>),
    Map(Box<DataType>, Box<DataType>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ApplicationId {
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub id: String,
    pub title: String,
    pub version: String,
    pub blocks: Vec<Box<dyn Block + Send>>,
    pub connections: Vec<Connection>,
    pub sources: Vec<Source>, //TODO - is it required
    pub sinks: Vec<Sink>, //TODO - is it required
    pub description: Option<String>,
    pub help: Option<String>,
}