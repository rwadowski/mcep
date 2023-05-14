use serde::{Deserialize, Serialize};
use crate::definition::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sink {
    id: Id,
    data_type: DataType,
}

impl Sink {
    pub fn new(id: Id, data_type: DataType) -> Sink {
        Sink {
            id,
            data_type,
        }
    }
}