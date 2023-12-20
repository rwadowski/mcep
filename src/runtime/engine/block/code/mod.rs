use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};
use chrono::Utc;
use std::collections::HashMap;

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
        let code = definition.code.clone();
        PythonCodeBlock {
            id: BlockId::new(deployment_id, id),
            definition,
            python_block: PythonBlock { code },
        }
    }
}
