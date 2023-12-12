use crate::engine::Data;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::{Eq, PartialEq, PartialOrd};
use types::deployment::source::SourceId;
use types::deployment::BlockId;

pub mod engine;
mod lib_test;
pub mod sink;
pub mod source;
mod util;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name {
    pub value: String,
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name {
            value: value.to_string(),
        }
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str: String = String::deserialize(deserializer)?;
        Ok(Name::from(str.as_str()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(with = "ts_milliseconds")]
    ts: DateTime<Utc>,
    name: Name,
    value: Data,
}

impl Message {
    fn key(&self) -> String {
        self.name.value.to_string()
    }

    fn as_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataFrame {
    origin: Origin,
    #[serde(with = "ts_milliseconds")]
    ts: DateTime<Utc>,
    name: Name,
    value: Data,
}

impl DataFrame {
    pub fn new(origin: Origin, ts: DateTime<Utc>, name: Name, value: Data) -> DataFrame {
        DataFrame {
            origin,
            ts,
            name,
            value,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

pub fn init() {
    pyo3::prepare_freethreaded_python();
}
