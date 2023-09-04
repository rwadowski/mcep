use definition::block::{Block as BlockDefinition, BlockType};
use definition::block::js::JsBlock as JsBlockDefinition;

use crate::DataFrame;
use crate::engine::BlockId;
use crate::engine::applications::ApplicationId;
use crate::engine::block::js::JsBlock;

mod js;
mod js_test;
mod mod_test;

pub(crate) trait Block {
    fn id(&self) -> BlockId;
    fn run(&mut self, df: DataFrame) -> Result<Vec<DataFrame>, String>;
}

pub(crate) fn new_block(application_id: ApplicationId, definition: Box<dyn BlockDefinition>) -> Result<Box<dyn Block>, String> {
    let block_type = definition.block_type();
    match block_type {
        BlockType::Js => {
            let def = as_js_block_definition(definition)?;
            Ok(Box::new(JsBlock::new(&application_id, def)))
        },
        _ => Err("unrecognized definition".to_string()),
    }
}

fn as_js_block_definition(definition: Box<dyn BlockDefinition>) -> Result<JsBlockDefinition, String> {
    match definition.as_any().downcast_ref::<JsBlockDefinition>() {
        Some(def) => Ok(def.clone()),
        None => Err("can't cast to js definition".to_string()),
    }
}
