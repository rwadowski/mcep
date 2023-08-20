use std::collections::HashMap;
use std::time::Instant;

use deno_core::{JsRuntime, RuntimeOptions};
use deno_core::url::Url;
use js_sandbox::Script;
use serde_derive::{Deserialize, Serialize};

use definition::block::js::JsBlock as JsBlockDefinition;

use crate::{DataFrame, Name, Origin};
use crate::engine::applications::ApplicationId;
use crate::engine::block::Block;
use crate::engine::{BlockId, Data};

#[derive(Serialize, Deserialize)]
struct ScriptInput {
    values: HashMap<Name, Data>
}

impl ScriptInput {
    fn new() -> ScriptInput {
        ScriptInput {
            values: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ScriptOutput {
    values: HashMap<Name, Data>
}

pub(crate) struct JsBlock {
    pub(crate) id: BlockId,
    pub(crate) definition: JsBlockDefinition,
    pub(crate) output_mappings: HashMap<Name, Name>,
    state: HashMap<Name, Data>,
}

impl Block for JsBlock {
    fn id(&self) -> BlockId {
        self.id.clone()
    }

    fn run(&mut self, df: DataFrame) -> Result<Vec<DataFrame>, String> {
        self.state.insert(df.name, df.payload);
        if self.state.len() != self.definition.inputs.len() {
            return Ok(Vec::new())
        }
        let mut script = Script::from_string(self.definition.code.as_str()).map_err(|e| e.to_string())?;
        let mut input = ScriptInput::new();
        for (name, value) in self.state.iter() {
            input.values.insert(name.clone(), value.clone());
        }
        let result: ScriptOutput = script.call("logic", &(input,))
            .map_err(|e| e.to_string())?;
        let origin = Origin::from(self.id());
        let frames: Vec<DataFrame> = result.values.iter().map(|tuple| {
            DataFrame::new(
                origin.clone(),
                Instant::now(),
                tuple.0.clone(),
                tuple.1.clone(),
            )
        }).collect();
        Ok(frames)
    }
}

async fn run__(definition:JsBlockDefinition, df: DataFrame) -> Result<DataFrame, String> {
    //TODO - it should be handled more reasonable
    let url = Url::parse("https://example.net").unwrap();
    // let module = deno_core::ModuleSource::new(
    //     deno_core::ModuleType::JavaScript,
    //     ModuleCode::from(self.definition.code.clone()),
    //     &url,
    // );
    let module_c = deno_core::ModuleCode::from(definition.code);
    let mut runtime = JsRuntime::new(
        RuntimeOptions::default(),
    );
    let mod_id = runtime.load_main_module(&url, Some(module_c)).await.unwrap();
    let result = runtime.mod_evaluate(mod_id);
    runtime.run_event_loop(false).await.unwrap();
    // let result = runtime.execute_script(
    //     "script",
    //     ModuleCode::from(self.definition.code.clone())
    // );
    Err("not_implemented".to_string())
}

impl JsBlock {
    pub(crate) fn new(application_id: &ApplicationId, definition: JsBlockDefinition) -> JsBlock {
        let mut output_mappings: HashMap<Name, Name> = HashMap::new();
        for output in definition.outputs.iter() {
            let name = Name::from(output.name.clone());
            output_mappings.insert(name.clone(), name.clone());
        }
        JsBlock {
            id: BlockId::new(application_id, &definition.id),
            definition,
            output_mappings,
            state: HashMap::new(),
        }
    }
}