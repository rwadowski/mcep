use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::definition::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Sink {
    pub id: Id,
    pub data_type: DataType,
}

impl Sink {
    pub fn new(id: Id, data_type: DataType) -> Sink {
        Sink {
            id,
            data_type,
        }
    }
}