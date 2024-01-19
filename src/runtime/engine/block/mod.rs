use actix::{Actor, ActorContext, Addr, Context, Handler, Message};
use log::{debug, error};
use std::collections::{HashMap, HashSet};

use crate::runtime::engine::block::code::PythonCodeBlock;
use crate::runtime::engine::Data;
use crate::runtime::sink::kafka::{KafkaSinkActor, KafkaSinkActorMessage};
use crate::runtime::{DataFrame, Name};
use crate::types::definition::block::code::github::Github;
use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
use crate::types::definition::block::{Block as BlockDefinition, BlockType};
use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};

pub mod code;
mod mod_test;

pub trait Block {
    fn id(&self) -> BlockId;
    fn run(&mut self, df: &HashMap<Name, Data>) -> Result<Vec<DataFrame>, String>;
}

pub fn new_block(
    definition: Box<dyn BlockDefinition>,
    deployment_id: DeploymentId,
    id: BlockInstanceId,
) -> Result<Box<dyn Block>, String> {
    let block_type = definition.block_type();
    match block_type {
        BlockType::CodeBlock => {
            let def = as_type::<CodeBlockDefinition>(definition)?;
            Ok(Box::new(PythonCodeBlock::new(
                def.source,
                deployment_id,
                id,
                def.inputs,
            )))
        }
        BlockType::Github => {
            let def = as_type::<Github>(definition)?;
            let src = def.source()?;
            Ok(Box::new(PythonCodeBlock::new(
                src,
                deployment_id,
                id,
                def.inputs,
            )))
        }
    }
}

fn as_type<T: Clone + 'static>(definition: Box<dyn BlockDefinition>) -> Result<T, String> {
    match definition.as_any().downcast_ref::<T>() {
        Some(def) => Ok(def.clone()),
        None => Err("can't cast to code block definition".to_string()),
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum BlockActorMessage {
    Process(Vec<DataFrame>),
    AddTargets(HashSet<Addr<BlockActor>>, HashSet<Addr<KafkaSinkActor>>),
    Stop,
}

pub struct BlockActor {
    state: HashMap<Name, Data>,
    block: Box<dyn Block>,
    blocks: HashSet<Addr<BlockActor>>,
    sinks: HashSet<Addr<KafkaSinkActor>>,
}

impl BlockActor {
    pub fn new(block: Box<dyn Block>) -> BlockActor {
        BlockActor {
            state: HashMap::new(),
            block,
            blocks: HashSet::new(),
            sinks: HashSet::new(),
        }
    }

    pub fn add_targets(
        &mut self,
        blocks: HashSet<Addr<BlockActor>>,
        sinks: HashSet<Addr<KafkaSinkActor>>,
    ) {
        self.blocks.extend(blocks);
        self.sinks.extend(sinks);
    }

    fn process(&mut self, data: Vec<DataFrame>) {
        let count = data.len();
        for frame in data {
            self.state.insert(frame.name, frame.value);
        }
        let result = self.block.run(&self.state);
        match result {
            Ok(list) => {
                debug!(
                    "processed block data {} frames for block {}",
                    count,
                    self.block.id()
                );
                self.blocks.iter().for_each(|addr| {
                    addr.do_send(BlockActorMessage::Process(list.clone()));
                });
                self.sinks.iter().for_each(|addr| {
                    addr.do_send(KafkaSinkActorMessage::Send(list.clone()));
                })
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
            BlockActorMessage::AddTargets(blocks, sinks) => self.add_targets(blocks, sinks),
            BlockActorMessage::Stop => ctx.stop(),
        }
    }
}
