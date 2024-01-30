mod mod_test;

use crate::services::github;
use crate::types::definition::block::code::CodeBlock;
use crate::types::definition::block::{Block, BlockType, Input, Output};
use crate::utils;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Github {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    source: Source,
}

impl Github {
    pub fn source(&self) -> Result<String, String> {
        Err("not implemented".to_string())
    }
}

#[typetag::serde]
impl Block for Github {
    fn block_type(&self) -> BlockType {
        BlockType::Github
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
        let v = Github {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            source: self.source.clone(),
        };
        Box::new(v)
    }

    fn as_json(&self) -> Result<Value, String> {
        let body = github::fetch_code(&self.source)?;
        let cb: Box<dyn Block> = Box::new(CodeBlock {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            source: body,
        });
        serde_json::to_value(cb).map_err(utils::to_string)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub owner: String,
    pub repository: String,
    pub token: String,
    pub path: String,
}
