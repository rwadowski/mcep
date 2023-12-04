use serde_derive::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::time::Instant;
use types::deployment::source::SourceId;
use types::deployment::BlockId;

use crate::engine::Data;

pub mod engine;
pub mod pool;
pub mod sink;
pub mod source;
mod util;

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
        self.origin.clone().to_string()
    }

    pub fn as_json(&self) -> String {
        String::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Origin {
    block: Option<BlockId>,
    source: Option<SourceId>,
}

impl From<BlockId> for Origin {
    fn from(value: BlockId) -> Self {
        Origin {
            block: Some(value),
            source: None,
        }
    }
}

impl From<SourceId> for Origin {
    fn from(value: SourceId) -> Self {
        Origin {
            block: None,
            source: Some(value),
        }
    }
}

impl Origin {
    fn to_string(self) -> String {
        if self.block.is_some() {
            return self.block.unwrap().to_string();
        }
        return self.source.unwrap().value;
    }
}

pub fn init() {
    pyo3::prepare_freethreaded_python();
}
