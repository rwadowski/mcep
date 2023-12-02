use std::collections::{HashMap, HashSet};

use actix::{Actor, ActorContext, Addr, Context, Handler, Message};
use log::{debug, error};

use crate::engine::block::code::PythonCodeBlock;
use crate::engine::Data;
use crate::{DataFrame, Name};
use types::definition::block::code::CodeBlock as CodeBlockDefinition;
use types::definition::block::{Block as BlockDefinition, BlockType};
use types::deployment::{BlockId, DeploymentId};

pub(crate) mod code;
mod mod_test;

pub trait Block {
    fn id(&self) -> BlockId;
    fn run(&mut self, df: &HashMap<Name, Data>) -> Result<Vec<DataFrame>, String>;
}

pub(crate) fn new_block(
    deployment_id: DeploymentId,
    definition: Box<dyn BlockDefinition>,
) -> Result<Box<dyn Block>, String> {
    let block_type = definition.block_type();
    match block_type {
        BlockType::Code => {
            let def = as_code_block_definition(definition)?;
            Ok(Box::new(PythonCodeBlock::new(&deployment_id, def)))
        }
        _ => Err("unrecognized definition".to_string()),
    }
}

fn as_code_block_definition(
    definition: Box<dyn BlockDefinition>,
) -> Result<CodeBlockDefinition, String> {
    match definition.as_any().downcast_ref::<CodeBlockDefinition>() {
        Some(def) => Ok(def.clone()),
        None => Err("can't cast to code block definition".to_string()),
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum BlockActorMessage {
    Process(Vec<DataFrame>),
    AddTargets(HashSet<Addr<BlockActor>>),
    Stop,
}

pub struct BlockActor {
    state: HashMap<Name, Data>,
    block: Box<dyn Block>,
    targets: HashSet<Addr<BlockActor>>,
}

impl BlockActor {
    pub fn new(block: Box<dyn Block>) -> BlockActor {
        BlockActor {
            state: HashMap::new(),
            block,
            targets: HashSet::new(),
        }
    }

    pub fn add_targets(&mut self, addresses: HashSet<Addr<BlockActor>>) {
        self.targets.extend(addresses);
    }

    fn process(&mut self, data: Vec<DataFrame>) {
        let count = data.len();
        for frame in data {
            self.state.insert(frame.name, frame.payload);
        }
        let result = self.block.run(&self.state);
        match result {
            Ok(list) => {
                debug!(
                    "processed block data {} frames for block {}",
                    count,
                    self.block.id()
                );
                for addr in self.targets.clone() {
                    let _ = addr.send(BlockActorMessage::Process(list.clone()));
                }
            }
            Err(err_string) => error!("failed to process message {}", err_string),
        }
    }
}

impl Actor for BlockActor {
    type Context = Context<Self>;
}

impl Handler<BlockActorMessage> for BlockActor {
    type Result = ();

    fn handle(&mut self, msg: BlockActorMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            BlockActorMessage::Process(data) => self.process(data),
            BlockActorMessage::AddTargets(targets) => self.add_targets(targets),
            BlockActorMessage::Stop => ctx.stop(),
        }
    }
}
