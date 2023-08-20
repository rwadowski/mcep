use std::any::Any;
use serde::{Deserialize, Serialize};
use crate::block::{Block, BlockType, Input, Output};
use crate::Id;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsBlock {
    pub id: Id,
    pub block_type: BlockType,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub code: String,
}

#[typetag::serde]
impl Block for JsBlock {
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

    fn as_any(&self) -> &dyn Any { self }
}