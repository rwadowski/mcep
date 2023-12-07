use std::collections::{HashMap, HashSet};

use actix::{Actor, ActorContext, Addr, Context, Handler, Message};

use types::definition::block::new_block_from_str;
use types::definition::block::Block as BlockDefinition;
use types::definition::{Definition, DefinitionId};
use types::deployment::sink::SinkId;
use types::deployment::source::SourceId;
use types::deployment::{BlockId, Deployment};

use crate::engine::block::{new_block, BlockActor, BlockActorMessage};
use crate::engine::router::Router;
use crate::sink::kafka::KafkaSinkActor;
use crate::DataFrame;

#[derive(Message)]
#[rtype(result = "()")]
pub enum FlowActorMessages {
    Process(DataFrame),
    Stop,
}

pub struct FlowActor {
    blocks: HashMap<BlockId, Addr<BlockActor>>,
    sources: HashMap<SourceId, HashSet<BlockId>>,
}

impl FlowActor {
    pub fn new(
        deployment: &Deployment,
        definitions: &HashMap<DefinitionId, Definition>,
        sinks: HashMap<SinkId, Addr<KafkaSinkActor>>,
    ) -> Result<Addr<FlowActor>, String> {
        let router = Router::new(&deployment.connections);
        let blocks: HashMap<BlockId, Addr<BlockActor>> =
            create_block_actors(deployment, definitions)?;
        init_actors(&blocks, &router, sinks);
        let flow_actor = FlowActor {
            blocks,
            sources: router.source_targets(),
        };
        Ok(flow_actor.start())
    }

    fn process(&self, df: &DataFrame) {
        match &df.origin.source {
            Some(source_id) => self.send_from_source(source_id, df),
            _ => {}
        }
    }

    fn send_from_source(&self, source_id: &SourceId, df: &DataFrame) {
        let block_ids = self
            .sources
            .get(&source_id)
            .unwrap_or(&HashSet::new())
            .clone();
        block_ids.iter().for_each(|block_id| {
            self.blocks.get(block_id).iter().for_each(|addr| {
                let frames = Vec::from([df.clone()]);
                addr.do_send(BlockActorMessage::Process(frames));
            })
        });
    }

    fn stop_workers(&mut self) {
        self.blocks.iter().for_each(|(_, addr)| {
            let _ = addr.do_send(BlockActorMessage::Stop);
        });
    }
}

fn option_to_set<T: Actor>(opt: Option<&Addr<T>>) -> HashSet<Addr<T>> {
    match opt {
        Some(v) => {
            let mut set = HashSet::new();
            set.insert(v.clone());
            set
        }
        None => HashSet::new(),
    }
}

fn create_block_actors(
    deployment: &Deployment,
    definitions: &HashMap<DefinitionId, Definition>,
) -> Result<HashMap<BlockId, Addr<BlockActor>>, String> {
    let mut blocks: HashMap<BlockId, Addr<BlockActor>> = HashMap::new();
    for deployed_block in deployment.blocks.iter() {
        let definition = definitions
            .get(&deployed_block.definition_id)
            .ok_or("no definition provided")?;
        let body = definition.body.to_string();
        let block_definition: Box<dyn BlockDefinition> = new_block_from_str(body.as_str())?;
        let block = new_block(block_definition, deployment.id, deployed_block.id)?;
        let block_actor = BlockActor::new(block);
        blocks.insert(deployed_block.id(), block_actor.start());
    }
    Ok(blocks)
}

fn init_actors(
    blocks: &HashMap<BlockId, Addr<BlockActor>>,
    router: &Router,
    sinks: HashMap<SinkId, Addr<KafkaSinkActor>>,
) {
    for (block_id, block) in blocks.iter() {
        let target_blocks: HashSet<Addr<BlockActor>> = router
            .block_targets(&block_id)
            .iter()
            .flat_map(|target| option_to_set(blocks.get(target)))
            .collect();
        let sink_blocks = router
            .sink_targets(&block_id)
            .iter()
            .flat_map(|target| option_to_set(sinks.get(target)))
            .collect();
        let msg = BlockActorMessage::AddTargets(target_blocks, sink_blocks);
        let _ = block.send(msg);
    }
}
impl Actor for FlowActor {
    type Context = Context<Self>;
}

impl Handler<FlowActorMessages> for FlowActor {
    type Result = ();

    fn handle(&mut self, msg: FlowActorMessages, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            FlowActorMessages::Process(df) => self.process(&df),
            FlowActorMessages::Stop => {
                self.stop_workers();
                ctx.stop()
            }
        }
    }
}
