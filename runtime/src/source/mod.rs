use serde_derive::{Deserialize, Serialize};

pub mod kafka;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct SourceId(pub String);

pub(crate) trait Source: std::fmt::Debug {
    fn id(&self) -> SourceId;
    fn run(&self) -> Result<(), String>;
}
