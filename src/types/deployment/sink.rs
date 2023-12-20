use crate::types::definition::DataType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SinkId {
    value: String,
}

impl From<&str> for SinkId {
    fn from(value: &str) -> Self {
        SinkId {
            value: value.to_string(),
        }
    }
}

impl Serialize for SinkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for SinkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str: String = String::deserialize(deserializer)?;
        Ok(SinkId::from(str.as_str()))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Sink {
    pub id: SinkId,
    pub data_type: DataType,
}

impl Sink {
    pub fn new(id: SinkId, data_type: DataType) -> Sink {
        Sink { id, data_type }
    }
}
