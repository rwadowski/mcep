use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use actix::dev::{MessageResponse, OneshotSender};
use actix::{Actor, Addr, Context, Handler, Message};
use log::error;
use serde::{Deserialize, Serialize};

use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::sink::SinkId;
use crate::types::deployment::{Deployment, DeploymentId};

use crate::runtime::engine::flow::{FlowActor, FlowActorMessages};
use crate::runtime::sink::kafka::KafkaSinkActor;
use crate::runtime::DataFrame;

mod block;
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
#[rtype(result = "EngineActorResponse")]
pub enum EngineActorMessage {
    Deploy(Deployment, Vec<Definition>),
    Undeploy(Deployment),
    Process(DataFrame),
}

pub enum EngineActorResponse {
    Succeed,
    Failed(String),
}

impl<A, M> MessageResponse<A, M> for EngineActorResponse
where
    A: Actor,
    M: Message<Result = EngineActorResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

pub struct EngineActor {
    flows: HashMap<DeploymentId, Addr<FlowActor>>,
    sinks: HashMap<SinkId, Addr<KafkaSinkActor>>,
}

impl EngineActor {
    pub fn new(sinks: HashMap<SinkId, Addr<KafkaSinkActor>>) -> EngineActor {
        EngineActor {
            flows: HashMap::new(),
            sinks,
        }
    }
    fn deploy(
        &mut self,
        deployment: &Deployment,
        definitions: &HashMap<DefinitionId, Definition>,
    ) -> Result<(), String> {
        let flow_actor = FlowActor::new(deployment, definitions, self.sinks.clone())?;
        self.flows.insert(deployment.id, flow_actor);
        Ok(())
    }

    fn undeploy(&mut self, deployment: &Deployment) {
        let removed = self.flows.remove(&deployment.id);
        removed.iter().for_each(|addr| {
            let _ = addr.send(FlowActorMessages::Stop);
        })
    }

    fn process(&mut self, df: DataFrame) {
        self.flows.iter().for_each(|(_, addr)| {
            addr.do_send(FlowActorMessages::Process(df.clone()));
        })
    }
}

impl Actor for EngineActor {
    type Context = Context<Self>;
}

impl Handler<EngineActorMessage> for EngineActor {
    type Result = EngineActorResponse;

    fn handle(&mut self, msg: EngineActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EngineActorMessage::Process(df) => {
                self.process(df);
                EngineActorResponse::Succeed
            }
            EngineActorMessage::Deploy(deployment, definitions) => {
                let definition_map: HashMap<DefinitionId, Definition> =
                    definitions.into_iter().map(|def| (def.id, def)).collect();
                let result = self.deploy(&deployment, &definition_map);
                match result {
                    Ok(()) => EngineActorResponse::Succeed,
                    Err(err) => {
                        error!("engine actor error {}", err);
                        EngineActorResponse::Failed(err)
                    }
                }
            }
            EngineActorMessage::Undeploy(deployment) => {
                self.undeploy(&deployment);
                EngineActorResponse::Succeed
            }
        }
    }
}
