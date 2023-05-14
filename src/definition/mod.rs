mod app;
mod block;
mod connection;
mod error;
mod mod_test;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id(pub String);
// pub struct Id {
//     value: String,
// }

impl Id {
    pub fn new(id: &str) -> Id {
        Id(id.to_string())
    }
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

mod definition {
    use std::collections::HashMap;

    enum Data {
        Boolean(bool),
        UnsignedInt(u32),
        SignedInt(i32),
        Float(f32),
        Text(String),
        Array(Vec<Data>),
        Map(HashMap<Data, Data>),
    }
}
