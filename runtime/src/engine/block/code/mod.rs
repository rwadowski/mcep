use std::collections::HashMap;
use std::time::Instant;

use types::definition::block::code::CodeBlock as CodeBlockDefinition;
use types::deployment::{BlockId, BlockInstanceId, DeploymentId};

use crate::engine::block::code::python::PythonBlock;
use crate::engine::block::Block;
use crate::engine::Data;
use crate::{DataFrame, Name, Origin};

mod mod_test;
pub mod python;
mod python_test;

pub struct PythonCodeBlock {
    pub(crate) id: BlockId,
    pub(crate) definition: CodeBlockDefinition,
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
                    Instant::now(),
                    Name::from(name.clone()),
                    data.clone(),
                )
            })
            .collect();
        Ok(frames)
    }
}

impl PythonCodeBlock {
    pub fn new(definition: CodeBlockDefinition, id: BlockInstanceId) -> PythonCodeBlock {
        let code = definition.code.clone();
        PythonCodeBlock {
            id: BlockId::new(definition.id, id),
            definition,
            python_block: PythonBlock { code },
        }
    }
}
