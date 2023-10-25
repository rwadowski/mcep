pub mod python;
mod python_test;
mod mod_test;

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use types::definition::block::code::CodeBlock as CodeBlockDefinition;
use types::deployment::DeploymentId;

use crate::{DataFrame, Name, Origin};
use crate::engine::block::Block;
use crate::engine::{BlockId, Data};
type Output = BTreeMap<String, Data>;
type Input = BTreeMap<String, Data>;

pub(crate) struct CodeBlock {
    pub(crate) id: BlockId,
    pub(crate) definition: CodeBlockDefinition,
    pub(crate) output_mappings: HashMap<Name, Name>,
    state: HashMap<Name, Data>,
}

impl Block for CodeBlock {
    fn id(&self) -> BlockId {
        self.id.clone()
    }

    fn run(&mut self, df: DataFrame) -> Result<Vec<DataFrame>, String> {
        self.state.insert(df.name, df.payload);
        if self.state.len() != self.definition.inputs.len() {
            return Ok(Vec::new())
        }
        // let mut script = Script::from_string(self.definition.code.as_str()).map_err(|e| e.to_string())?;
        let mut input: HashMap<String, Data> = HashMap::new();
        for (name, value) in self.state.iter() {
            input.insert(name.value.clone(), value.clone());
        }
        let python_block = python::PythonBlock {
            code: self.definition.code.clone()
        };
        let result = python_block.run_python_code(input)?;
        let origin = Origin::from(self.id());
        let frames: Vec<DataFrame> = result.iter().map(|tuple| {
            DataFrame::new(
                origin.clone(),
                Instant::now(),
                Name::from(tuple.0.clone()),
                tuple.1.clone(),
            )
        }).collect();
        Ok(frames)
    }
}

impl CodeBlock {
    pub(crate) fn new(deployment_id: &DeploymentId, definition: CodeBlockDefinition) -> CodeBlock {
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        for output in definition.outputs.iter() {
            let name = Name::from(output.name.clone());
            output_mappings.insert(name.clone(), name.clone());
        }
        CodeBlock {
            id: BlockId::new(deployment_id, &definition.id),
            definition,
            output_mappings,
            state: HashMap::new(),
        }
    }
}