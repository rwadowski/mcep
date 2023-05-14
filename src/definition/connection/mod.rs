use serde::{Deserialize, Serialize};
use crate::definition::connection::junction::Junction;
use crate::definition::error::DefinitionError;

pub mod sink;
pub mod source;
pub mod junction;
mod junction_test;
mod mod_test;
mod sink_test;
mod source_test;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    from: Junction,
    to: Junction,
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
