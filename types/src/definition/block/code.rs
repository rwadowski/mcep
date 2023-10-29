use std::any::Any;
use serde::{Deserialize, Serialize};
use crate::definition::block::{Block, BlockType, CodeBlockType, Input, Output};
use crate::definition::Id;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub id: Id,
    pub block_type: BlockType,
    pub code_block_type: CodeBlockType,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub code: String,
}

#[typetag::serde]
impl Block for CodeBlock {
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

    fn clone_box(&self) -> Box<dyn Block> {
        let block = CodeBlock {
            id: self.id.clone(),
            block_type: self.block_type.clone(),
            code_block_type: self.code_block_type.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            code: self.code.clone(),
        };
        Box::new(block)
    }
}