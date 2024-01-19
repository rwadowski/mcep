pub mod github;
mod mod_test;

use crate::types::definition::block::{Block, BlockType, Input, Output};
use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub source: String,
}

#[typetag::serde]
impl Block for CodeBlock {
    fn block_type(&self) -> BlockType {
        BlockType::CodeBlock
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
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            source: self.source.clone(),
        };
        Box::new(block)
    }

    fn as_json(&self) -> Result<Value, String> {
        let boxed: Box<dyn Block> = Box::new(self.clone());
        serde_json::to_value(boxed).map_err(utils::to_string)
    }
}
