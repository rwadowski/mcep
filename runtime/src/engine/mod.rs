use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use actix::{Actor, Addr, Context, Handler, Message};
use serde_derive::{Deserialize, Serialize};

use types::definition::block::new_block_from_str;
use types::definition::block::Block as BlockDefinition;
use types::definition::Definition;
use types::deployment::{BlockId, Deployment, DeploymentId};

use crate::engine::block::{new_block, BlockActor};
use crate::engine::flow::FlowActor;
use crate::engine::router::Router;
use crate::sink::kafka::KafkaSinkActor;
use crate::DataFrame;

mod block;
pub mod engine;
mod flow;
pub mod router;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Boolean(bool),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
    Text(String),
    Array(Vec<Data>),
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        return &self;
    }
}
impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Data::Boolean(b) => b.hash(state),
            Data::UnsignedInt(i) => i.hash(state),
            Data::SignedInt(i) => i.hash(state),
            Data::Text(s) => s.hash(state),
            Data::Float(f) => f.to_string().hash(state),
            Data::Array(arr) => arr.hash(state),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum EngineActorMessage {
    Deploy(Deployment, Vec<Definition>),
    Undeploy(Deployment),
    Process(DataFrame),
}

pub struct EngineActor {
    flows: HashMap<DeploymentId, Addr<FlowActor>>,
    sinks: Vec<Addr<KafkaSinkActor>>,
}

impl EngineActor {
    pub fn new(sink: Addr<KafkaSinkActor>) -> EngineActor {
        let mut sinks: Vec<Addr<KafkaSinkActor>> = Vec::new();
        sinks.push(sink);
        EngineActor {
            flows: HashMap::new(),
            sinks,
        }
    }
    fn deploy(
        &mut self,
        deployment: &Deployment,
        definitions: &Vec<Definition>,
    ) -> Result<(), String> {
        let mut blocks: HashMap<BlockId, Addr<BlockActor>> = HashMap::new();
        let router = Router::new(&deployment.connections);
        for definition in definitions.iter() {
            let block_definition: Box<dyn BlockDefinition> =
                new_block_from_str(definition.body.to_string().as_str())?;
            let block = new_block(deployment.id, block_definition)?;
            let block_id = block.id();
            let block_actor = BlockActor::new(block);
            blocks.insert(block_id, block_actor.start());
        }
        let flow_actor = FlowActor::new(blocks, self.sinks.clone(), router);
        self.flows.insert(deployment.id, flow_actor);
        Ok(())
    }

    fn undeploy(&mut self, deployment: &Deployment) {
        if let Some(flow_address) = self.flows.get(&deployment.id) {
            self.flows.remove(&deployment.id);
        }
    }

    fn process(&mut self, df: &DataFrame) {}
}

impl Actor for EngineActor {
    type Context = Context<Self>;
}

impl Handler<EngineActorMessage> for EngineActor {
    type Result = ();

    fn handle(&mut self, msg: EngineActorMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EngineActorMessage::Process(df) => self.process(&df),
            EngineActorMessage::Deploy(deployment, connections) => {
                let _ = self.deploy(&deployment, &connections);
            }
            EngineActorMessage::Undeploy(deployment) => self.undeploy(&deployment),
        }
    }
}
