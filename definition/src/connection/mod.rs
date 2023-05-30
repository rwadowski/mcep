use serde::{Deserialize, Serialize};
use crate::connection::junction::Junction;

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

