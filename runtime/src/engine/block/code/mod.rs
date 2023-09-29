mod python;

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use definition::block::code::CodeBlock as CodeBlockDefinition;

use crate::{DataFrame, Name, Origin};
use crate::engine::applications::ApplicationId;
use crate::engine::block::Block;
use crate::engine::{BlockId, Data};
type Output = BTreeMap<String, Data>;
type Input = BTreeMap<String, Data>;

fn new_input() -> Input {
    let mut m: BTreeMap<String, Data> = BTreeMap::new();
    m.insert("x".to_string(), Data::Text("txt".to_string()));
    m.insert("y".to_string(), Data::Boolean(true));
    m
}


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
        let mut input: BTreeMap<String, Data> = BTreeMap::new();
        for (name, value) in self.state.iter() {
            input.insert(name.value.clone(), value.clone());
        }
        // let result: Output = script.call("logic", &input)
        //     .map_err(|e| e.to_string())?;
        let origin = Origin::from(self.id());
        // let frames: Vec<DataFrame> = result.iter().map(|tuple| {
        //     DataFrame::new(
        //         origin.clone(),
        //         Instant::now(),
        //         Name::from(tuple.0.clone()),
        //         tuple.1.clone(),
        //     )
        // }).collect();
        // Ok(frames)
        Err("not implemented".to_string())
    }
}

impl CodeBlock {
    pub(crate) fn new(application_id: &ApplicationId, definition: CodeBlockDefinition) -> CodeBlock {
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        for output in definition.outputs.iter() {
            let name = Name::from(output.name.clone());
            output_mappings.insert(name.clone(), name.clone());
        }
        CodeBlock {
            id: BlockId::new(application_id, &definition.id),
            definition,
            output_mappings,
            state: HashMap::new(),
        }
    }
}