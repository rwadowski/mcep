pub mod js;
mod mod_test;
mod js_test;

use std::any::Any;
use serde::{Deserialize, Serialize};
use crate::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    Js,
}

#[typetag::serde(tag = "type")]
pub trait Block: std::fmt::Debug {
    fn id(&self) -> Id;
    fn block_type(&self) -> BlockType;
    fn inputs(&self) -> Vec<Input>;
    fn outputs(&self) -> Vec<Output>;
    fn as_any(&self) -> &dyn Any;
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
