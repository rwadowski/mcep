use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use types::definition::block::new_block_from_str;
use types::definition::block::Block as BlockDefinition;
use types::definition::Definition;
use types::deployment::{BlockId, Deployment, DeploymentId};

use crate::engine::block::{new_block, BlockActor};
use crate::engine::flow::FlowActor;
use crate::engine::router::Router;
use crate::sink::kafka::KafkaSinkActor;
use crate::DataFrame;

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
