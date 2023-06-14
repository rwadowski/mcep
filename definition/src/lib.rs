pub mod block;
pub mod connection;
pub mod error;
pub mod lib_test;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use sqlx::FromRow;
use crate::block::Block;
use crate::connection::Connection;
use crate::connection::sink::Sink;
use crate::connection::source::Source;

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

enum Data {
    Boolean(bool),
    UnsignedInt(u32),
    SignedInt(i32),
    Float(f32),
    Text(String),
    Array(Vec<Data>),
    Map(HashMap<Data, Data>),
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub id: String,
    pub title: String,
    pub version: String,
    pub blocks: Vec<Box<dyn Block>>,
    pub connections: Vec<Connection>,
    pub sources: Vec<Source>,
    pub sinks: Vec<Sink>,
    pub description: Option<String>,
    pub help: Option<String>,
}