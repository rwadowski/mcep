use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};
use chrono::Utc;
use std::collections::HashMap;

use crate::runtime::engine::block::code::python::PythonBlock;
use crate::runtime::engine::block::Block;
use crate::runtime::engine::Data;
use crate::runtime::{DataFrame, Name, Origin};
use crate::types::definition::block::{Dependency, Input};

mod mod_test;
pub mod python;
mod python_test;

pub struct PythonCodeBlock {
    pub id: BlockId,
    python_block: PythonBlock,
    inputs: Vec<Input>,
}

impl Block for PythonCodeBlock {
    fn id(&self) -> BlockId {
        self.id.clone()
    }

    fn run(&mut self, df: &HashMap<Name, Data>) -> Result<Vec<DataFrame>, String> {
        if df.len() != self.inputs.len() {
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
        source: String,
        deployment_id: DeploymentId,
        id: BlockInstanceId,
        inputs: Vec<Input>,
        dependencies: Vec<Dependency>,
    ) -> PythonCodeBlock {
        PythonCodeBlock {
            id: BlockId::new(deployment_id, id),
            python_block: PythonBlock::new(source, dependencies),
            inputs,
        }
    }
}
