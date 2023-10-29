use types::definition::block::{Block as BlockDefinition, BlockType};
use types::definition::block::code::CodeBlock as CodeBlockDefinition;
use types::deployment::DeploymentId;

use crate::DataFrame;
use crate::engine::BlockId;
use crate::engine::block::code::PythonCodeBlock;

pub(crate) mod code;
mod mod_test;

pub(crate) trait Block {
    fn id(&self) -> BlockId;
    fn run(&mut self, df: DataFrame) -> Result<Vec<DataFrame>, String>;
}

pub(crate) fn new_block(deployment_id: DeploymentId, definition: Box<dyn BlockDefinition>) -> Result<Box<dyn Block>, String> {
    let block_type = definition.block_type();
    match block_type {
        BlockType::Code => {
            let def = as_code_block_definition(definition)?;
            Ok(Box::new(PythonCodeBlock::new(&deployment_id, def)))
        }
        _ => Err("unrecognized definition".to_string()),
    }
}

fn as_code_block_definition(definition: Box<dyn BlockDefinition>) -> Result<CodeBlockDefinition, String> {
    match definition.as_any().downcast_ref::<CodeBlockDefinition>() {
        Some(def) => Ok(def.clone()),
        None => Err("can't cast to code block definition".to_string()),
    }
}

