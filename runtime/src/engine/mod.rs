use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::os::macos::raw::stat;
use serde_derive::{Deserialize, Serialize};
use definition::Id;
use crate::engine::applications::ApplicationId;
use crate::engine::Data::Boolean;

pub mod engine;
mod applications;
mod router;
mod block;

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct BlockId(pub String);

impl BlockId {
    fn new(application_id: &ApplicationId, id: &Id) -> BlockId {
        BlockId(
            application_id.0.clone() + "." + id.0.as_str(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Data {
    Boolean(bool),
    UnsignedInt(u32),
    SignedInt(i32),
    //TODO - add floats
    // Float(f64),
    Text(String),
    Array(Vec<Data>),
    //TODO - add hashes
    // Map(HashMap<String, Data>),
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Data::Boolean(b) => b.hash(state),
            Data::UnsignedInt(i) => i.hash(state),
            Data::SignedInt(i) => i.hash(state),
            Data::Text(s) => s.hash(state),
            Data::Array(arr) => arr.hash(state),
        }
    }
}