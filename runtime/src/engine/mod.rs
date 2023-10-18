use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use serde_derive::{Deserialize, Serialize};
use definition::Id;
use crate::engine::applications::ApplicationId;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Boolean(bool),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
    Text(String),
    Array(Vec<Data>),
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        return &self
    }
}
impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Data::Boolean(b) => b.hash(state),
            Data::UnsignedInt(i) => i.hash(state),
            Data::SignedInt(i) => i.hash(state),
            Data::Text(s) => s.hash(state),
            Data::Float(f) => f.to_string().hash(state),
            Data::Array(arr) => arr.hash(state),
        }
    }
}