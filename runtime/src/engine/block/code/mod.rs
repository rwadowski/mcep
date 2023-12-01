mod mod_test;
pub mod python;
mod python_test;

use crate::engine::block::code::python::PythonBlock;
use crate::engine::block::Block;
use crate::engine::{BlockId, Data};
use crate::{DataFrame, Name};
use std::collections::HashMap;
use std::time::Instant;
use types::definition::block::code::CodeBlock as CodeBlockDefinition;
use types::deployment::DeploymentId;

pub struct PythonCodeBlock {
    pub(crate) id: BlockId,
    pub(crate) definition: CodeBlockDefinition,
    state: HashMap<Name, Data>,
    python_block: PythonBlock,
}

impl Block for PythonCodeBlock {
    fn id(&self) -> BlockId {
        self.id.clone()
    }

    fn run(&mut self, df: &DataFrame) -> Result<Vec<DataFrame>, String> {
        self.state.insert(df.name.clone(), df.payload.clone());
        if self.state.len() != self.definition.inputs.len() {
            return Ok(Vec::new());
        }
        // let mut script = Script::from_string(self.definition.code.as_str()).map_err(|e| e.to_string())?;
        let mut input: HashMap<String, Data> = HashMap::new();
        for (name, value) in self.state.iter() {
            input.insert(name.value.clone(), value.clone());
        }
        let result = self.python_block.run(input)?;
        let origin = self.id.clone();
        let frames: Vec<DataFrame> = result
            .iter()
            .map(|tuple| {
                DataFrame::new(
                    origin.clone(),
                    Instant::now(),
                    Name::from(tuple.0.clone()),
                    tuple.1.clone(),
                )
            })
            .collect();
        Ok(frames)
    }
}

impl PythonCodeBlock {
    pub fn new(deployment_id: &DeploymentId, definition: CodeBlockDefinition) -> PythonCodeBlock {
        let code = definition.code.clone();
        PythonCodeBlock {
            id: BlockId::new(deployment_id, &definition.id),
            definition,
            state: HashMap::new(),
            python_block: PythonBlock { code },
        }
    }
}
