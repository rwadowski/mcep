use serde::{Deserialize, Serialize};
use crate::definition::error::DefinitionError;
use crate::deployment::connection::junction::BlockJunction;

pub mod junction;
mod junction_test;
mod mod_test;

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