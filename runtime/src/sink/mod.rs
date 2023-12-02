use serde_derive::{Deserialize, Serialize};

pub mod kafka;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct SinkId(pub String);
