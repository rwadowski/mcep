use serde::{Deserialize, Serialize};
use crate::definition::error::DefinitionError;
use crate::deployment::connection::junction::Junction;

pub mod junction;
mod junction_test;
mod mod_test;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Connection {
    pub from: Junction,
    pub to: Junction,
}

impl Connection {
    fn new(from: Junction, to: Junction) -> Result<Connection, DefinitionError> {
        if from.data_type != to.data_type {
            return Err(DefinitionError::IncorrectJunctionDataTypes);
        }
        Ok(
            Connection { from, to }
        )
    }
}
