use serde::{Deserialize, Serialize};
use crate::definition::error::DefinitionError;
use crate::deployment::connection::junction::{BlockJunction, DefinitionJunction};

pub mod junction;
mod junction_test;
mod mod_test;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DefinitionConnection {
    pub from: DefinitionJunction,
    pub to: DefinitionJunction,
}

impl DefinitionConnection {
    fn new(from: DefinitionJunction, to: DefinitionJunction) -> Result<DefinitionConnection, DefinitionError> {
        if from.data_type != to.data_type {
            return Err(DefinitionError::IncorrectJunctionDataTypes);
        }
        Ok(
            DefinitionConnection { from, to }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct BlockConnection {
    pub from: BlockJunction,
    pub to: BlockJunction,
}

impl BlockConnection {
    fn new(from: BlockJunction, to: BlockJunction) -> Result<BlockConnection, DefinitionError> {
        if from.data_type != to.data_type {
            return Err(DefinitionError::IncorrectJunctionDataTypes);
        }
        Ok(
            BlockConnection { from, to }
        )
    }
}