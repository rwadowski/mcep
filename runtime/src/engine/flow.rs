use std::collections::{HashMap, HashSet};

use actix::{Actor, ActorContext, Addr, Context, Handler, Message};

use types::deployment::BlockId;

use crate::engine::block::{BlockActor, BlockActorMessage};
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
    sinks: Vec<Addr<KafkaSinkActor>>,
    router: Router,
}

impl FlowActor {
    pub fn new(
        blocks: HashMap<BlockId, Addr<BlockActor>>,
        sinks: Vec<Addr<KafkaSinkActor>>,
        router: Router,
    ) -> Addr<FlowActor> {
        for (block_id, block) in blocks.iter() {
            let addresses: HashSet<Addr<BlockActor>> = router
                .targets(&block_id)
                .iter()
                .flat_map(|target| option_to_set(blocks.get(target)))
                .collect();
            block.send(BlockActorMessage::AddTargets(addresses));
        }
        let actor = FlowActor {
            blocks,
            sinks,
            router,
        };
        actor.start()
    }

    fn process(&mut self, df: &DataFrame) {
        panic!("not implemented") //TODO how to send results to sinks - probably add them to block actor, flow can have multiple exits ?
    }

    fn stop_workers(&mut self) {
        self.blocks.iter().for_each(|(_, addr)| {
            let _ = addr.send(BlockActorMessage::Stop);
        });
    }
}

fn option_to_set(opt: Option<&Addr<BlockActor>>) -> HashSet<Addr<BlockActor>> {
    match opt {
        Some(v) => {
            let mut set = HashSet::new();
            set.insert(v.clone());
            set
        }
        None => HashSet::new(),
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
