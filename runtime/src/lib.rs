pub mod engine;
pub mod pool;
pub mod sink;
pub mod source;
mod util;

use crate::engine::Data;
use serde_derive::{Deserialize, Serialize};
use source::SourceId;
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::time::Instant;
use types::deployment::BlockId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceId(pub String);

impl From<SourceId> for InstanceId {
    fn from(value: SourceId) -> Self {
        InstanceId(value.0)
    }
}

impl From<BlockId> for InstanceId {
    fn from(block_id: BlockId) -> Self {
        InstanceId(block_id.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Name {
    pub value: String,
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Name { value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataFrame {
    origin: BlockId,
    ts: Instant,
    name: Name,
    payload: Data,
}

impl DataFrame {
    pub fn new(origin: BlockId, ts: Instant, name: Name, payload: Data) -> DataFrame {
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

pub fn init() {
    pyo3::prepare_freethreaded_python();
}
