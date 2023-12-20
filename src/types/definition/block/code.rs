use crate::types::definition::block::{Block, BlockType, CodeBlockType, Input, Output};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub block_type: BlockType,
    pub code_block_type: CodeBlockType,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub code: String,
}

#[typetag::serde]
impl Block for CodeBlock {
    fn block_type(&self) -> BlockType {
        self.block_type.clone()
    }

    fn inputs(&self) -> Vec<Input> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<Output> {
        self.outputs.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Block> {
        let block = CodeBlock {
            block_type: self.block_type.clone(),
            code_block_type: self.code_block_type.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            code: self.code.clone(),
        };
        Box::new(block)
    }
}
