use crate::types::definition::DataType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Type, FromRow, Hash)]
pub struct SourceId {
    pub value: String,
}

impl From<&str> for SourceId {
    fn from(value: &str) -> Self {
        SourceId {
            value: value.to_string(),
        }
    }
}

impl Serialize for SourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

impl<'de> Deserialize<'de> for SourceId {
    fn deserialize<'d, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str: String = String::deserialize(deserializer)?;
        Ok(SourceId::from(str.as_str()))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, FromRow)]
pub struct Source {
    pub id: SourceId,
    pub data_type: DataType,
}

impl Source {
    pub fn new(id: SourceId, data_type: DataType) -> Source {
        Source { id, data_type }
    }
}
