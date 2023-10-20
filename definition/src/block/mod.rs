pub mod code;
mod mod_test;
mod code_test;

use std::any::Any;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    Code,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeBlockType {
    Js,
    Python,
}

#[typetag::serde(tag = "type")]
pub trait Block: Send + Debug {
    fn id(&self) -> Id;
    fn block_type(&self) -> BlockType;
    fn inputs(&self) -> Vec<Input>;
    fn outputs(&self) -> Vec<Output>;
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Block>;
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
