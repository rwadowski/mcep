pub mod js;
mod mod_test;
mod js_test;

use serde::{Deserialize, Serialize};
use crate::definition::{DataType, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockType {
    Js,
}

pub trait Block {
    fn id(&self) -> Id;
    fn block_type(&self) -> BlockType;
    fn inputs(&self) -> Vec<Input>;
    fn outputs(&self) -> Vec<Output>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub data_type: DataType,
}

impl Input {
    fn new(name: String, dt: DataType) -> Input {
        Input {
            name,
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
    fn new(name: String, dt: DataType) -> Output {
        Output {
            name,
            data_type: dt,
        }
    }
}