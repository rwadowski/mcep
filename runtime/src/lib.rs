pub mod pool;
pub mod source;
pub mod sink;
mod util;
pub mod engine;

use std::time::Instant;
use source::SourceId;
use std::cmp::{Eq, PartialOrd, PartialEq};
use serde_derive::{Deserialize, Serialize};
use crate::engine::{BlockId, Data};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceId(pub String);

impl From<SourceId> for InstanceId {
    fn from(value: SourceId) -> Self {
        InstanceId(value.0)
    }
}

impl From<BlockId> for InstanceId {
    fn from(value: BlockId) -> Self {
        InstanceId(value.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Origin(String);

impl From<InstanceId> for Origin {
    fn from(value: InstanceId) -> Self {
        Origin(value.0)
    }
}

impl From<BlockId> for Origin {
    fn from(value: BlockId) -> Self {
       Origin(value.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Name {
    pub value: String
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Name{ value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataFrame {
    origin: Origin,
    ts: Instant,
    name: Name,
    payload: Data,
}

impl DataFrame {
    pub fn new(origin: Origin, ts: Instant, name: Name, payload: Data) -> DataFrame {
        DataFrame {
            origin,
            ts,
            name,
            payload,
        }
    }

    pub fn key(&self) -> String {
        self.origin.0.clone()
    }

    pub fn as_json(&self) -> String {
        String::new()
    }
}