use serde::{Deserialize, Serialize};
use crate::definition::error::DefinitionError;
use crate::definition::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Junction {
    pub block: Id,
    pub id: Id,
    pub data_type: DataType,
}

impl Junction {
    pub fn new(str: &str, data_type: DataType) -> Result<Junction, DefinitionError> {
        let res: Vec<&str> = str.split('.').collect();
        if res.len() != 2 {
            return Err(DefinitionError::IncorrectJunctionString);
        }
        Ok(Junction {
            block: Id::new(res[0]),
            id: Id::new(res[1]),
            data_type,
        })
    }
}