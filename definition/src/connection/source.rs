use serde::{Deserialize, Serialize};
use crate::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub id: Id,
    pub data_type: DataType,
}

impl Source {
    pub fn new(id: Id, data_type: DataType) -> Source {
        Source {
            id,
            data_type,
        }
    }
}

