pub mod code;
mod mod_test;

use crate::types::definition::DataType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    CodeBlock,
    Github,
}

#[typetag::serde(tag = "type")]
pub trait Block: Send + Debug {
    fn block_type(&self) -> BlockType;
    fn inputs(&self) -> Vec<Input>;
    fn outputs(&self) -> Vec<Output>;
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Block>;
    fn as_json(&self) -> Result<Value, String>;
}

pub fn new_block_from_str(s: &str) -> Result<Box<dyn Block>, String> {
    serde_json::from_str(&s).map_err(|err| err.to_string())
}

impl Clone for Box<dyn Block> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub data_type: DataType,
}

impl Input {
    pub fn new(name: &str, dt: DataType) -> Input {
        Input {
            name: name.to_string(),
            data_type: dt,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub data_type: DataType,
}

impl Output {
    pub fn new(name: &str, dt: DataType) -> Output {
        Output {
            name: name.to_string(),
            data_type: dt,
        }
    }
}
