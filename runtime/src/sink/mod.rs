use serde_derive::{Deserialize, Serialize};

pub mod kafka;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct SinkId(pub String);

pub(crate) trait Sink: std::fmt::Debug {
    fn id(&self) -> SinkId;
    fn run(&self) -> Result<(), String>;
}