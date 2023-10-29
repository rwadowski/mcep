use serde::{Deserialize, Serialize};
use crate::definition::error::DefinitionError;
use crate::definition::{DataType, DefinitionId, Id};
use crate::deployment::BlockId;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DefinitionJunction {
    pub block: DefinitionId,
    pub id: Id,
    pub data_type: DataType,
}

impl DefinitionJunction {
    pub fn new(str: &str, data_type: DataType) -> Result<DefinitionJunction, DefinitionError> {
        let res: Vec<&str> = str.split('.').collect();
        if res.len() != 2 {
            return Err(DefinitionError::IncorrectJunctionString);
        }
        let definition_id: DefinitionId = res[0].parse().map_err(|_| DefinitionError::IncorrectJunctionString)?;
        Ok(DefinitionJunction {
            block: definition_id,
            id: Id::new(res[1]),
            data_type,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BlockJunction {
    pub block: BlockId,
    pub data_type: DataType,
}

impl BlockJunction {

    pub fn new(block_id: BlockId, data_type: DataType) -> BlockJunction {
        BlockJunction {
            block: block_id,
            data_type,
        }
    }
}