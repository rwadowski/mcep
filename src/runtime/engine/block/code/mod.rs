use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::runtime::engine::block::code::python::PythonBlock;
use crate::runtime::engine::block::Block;
use crate::runtime::engine::Data;
use crate::runtime::{DataFrame, Name, Origin};

mod mod_test;
pub mod python;
mod python_test;

pub struct PythonCodeBlock {
    pub id: BlockId,
    pub definition: CodeBlockDefinition,
    // state: HashMap<Name, Data>,
    python_block: PythonBlock,
}

impl Block for PythonCodeBlock {
    fn id(&self) -> BlockId {
        self.id.clone()
    }

    fn run(&mut self, df: &HashMap<Name, Data>) -> Result<Vec<DataFrame>, String> {
        if df.len() != self.definition.inputs.len() {
            return Ok(Vec::new());
        }
        let mut input: HashMap<String, Data> = HashMap::new();
        for (name, value) in df.iter() {
            input.insert(name.value.clone(), value.clone());
        }
        let result = self.python_block.run(input)?;
        let frames: Vec<DataFrame> = result
            .iter()
            .map(|(name, data)| {
                DataFrame::new(
                    Origin::from(self.id.clone()),
                    Utc::now(),
                    Name::from(name.as_str()),
                    data.clone(),
                )
            })
            .collect();
        Ok(frames)
    }
}

impl PythonCodeBlock {
    pub fn new(
        definition: CodeBlockDefinition,
        deployment_id: DeploymentId,
        id: BlockInstanceId,
    ) -> PythonCodeBlock {
        let src = definition.source.clone();
        PythonCodeBlock {
            id: BlockId::new(deployment_id, id),
            definition,
            python_block: PythonBlock { source: src },
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Code: Send + Debug {
    fn code(&self) -> Result<String, String>;

    fn clone_box(&self) -> Box<dyn Code>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlainCode {
    code: String,
}

impl Clone for Box<dyn Code> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[typetag::serde]
impl Code for PlainCode {
    fn code(&self) -> Result<String, String> {
        Ok(self.code.clone())
    }

    fn clone_box(&self) -> Box<dyn Code> {
        let raw_code = PlainCode {
            code: self.code.clone(),
        };
        Box::new(raw_code)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubCode {
    pub owner: String,
    pub repository: String,
    pub path: String,
    pub token: String, //TODO - store it somewhere else ex. env ?
}

#[typetag::serde]
impl Code for GithubCode {
    fn code(&self) -> Result<String, String> {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Code> {
        todo!()
    }
}
