pub mod block;
pub mod connection;
pub mod error;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use sqlx::FromRow;

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

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::DataType;

    #[test_case(DataType::Boolean, "\"Boolean\""; "serialization of boolean is correct")]
    #[test_case(DataType::UnsignedInt, "\"UnsignedInt\""; "serialization of unsigned int is correct")]
    #[test_case(DataType::SignedInt, "\"SignedInt\""; "serialization of signed int is correct")]
    #[test_case(DataType::FloatType, "\"FloatType\""; "serialization of float is correct")]
    #[test_case(DataType::Text, "\"Text\""; "serialization of text is correct")]
    #[test_case(DataType::Array(Box::from(DataType::Text)), "{\"Array\":\"Text\"}"; "serialization of array is correct")]
    #[test_case(DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)), "{\"Map\":[\"Text\",\"Text\"]}"; "serialization of map is correct")]
    fn test_data_type_serialization(dt: DataType, expected: &str) {
        let result = serde_json::to_string(&dt);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected.to_string(), result.unwrap());
    }

    #[test_case("\"Boolean\"", DataType::Boolean; "deserialization of boolean is correct")]
    #[test_case("\"UnsignedInt\"", DataType::UnsignedInt; "deserialization of unsigned int is correct")]
    #[test_case("\"SignedInt\"", DataType::SignedInt; "deserialization of signed int is correct")]
    #[test_case("\"FloatType\"", DataType::FloatType; "deserialization of float is correct")]
    #[test_case("\"Text\"", DataType::Text; "deserialization of text is correct")]
    #[test_case("{\"Array\":\"Text\"}", DataType::Array(Box::from(DataType::Text)); "deserialization of array is correct")]
    #[test_case("{\"Map\":[\"Text\",\"Text\"]}", DataType::Map(Box::from(DataType::Text), Box::from(DataType::Text)); "deserialization of map is correct")]
    fn test_data_type_deserialization(data: &str, expected: DataType) {
        let result = serde_json::from_str::<DataType>(data);
        assert_eq!(true, result.is_ok());
        assert_eq!(expected, result.unwrap());
    }
}
