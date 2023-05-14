use serde::{Deserialize, Serialize};
use crate::definition::block::{Block, BlockType, Input, Output};
use crate::definition::Id;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Js {
    pub id: Id,
    pub block_type: BlockType,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub code: String,
}

impl Js {}

impl Block for Js {
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn block_type(&self) -> BlockType {
        self.block_type.clone()
    }

    fn inputs(&self) -> Vec<Input> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<Output> {
        self.outputs.clone()
    }
}