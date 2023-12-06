use crate::engine::{EngineActor, EngineActorMessage};
use crate::source::kafka::KafkaSource;
use crate::DataFrame;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, WrapFuture};
use std::time::Duration;

pub mod kafka;

pub trait Source {
    fn fetch(&mut self) -> Result<Vec<DataFrame>, String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum SourceActorMessage {
    Poll,
}

pub struct SourceActor {
    source: Box<dyn Source>,
    engine: Addr<EngineActor>,
}

impl SourceActor {
    pub fn new(engine: Addr<EngineActor>) -> Result<SourceActor, String> {
        let source = KafkaSource::new()?;
        Ok(SourceActor { source, engine })
    }
}

impl Actor for SourceActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(
            Duration::from_millis(1000),
            |_, c: &mut Context<SourceActor>| {
                let _ = c.address().do_send(SourceActorMessage::Poll);
            },
        );
    }
}

impl Handler<SourceActorMessage> for SourceActor {
    type Result = ();

    fn handle(&mut self, msg: SourceActorMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SourceActorMessage::Poll => {
                let frames = self.source.fetch().unwrap();
                frames.into_iter().for_each(|frame| {
                    self.engine.do_send(EngineActorMessage::Process(frame));
                })
            }
        }
    }
}
