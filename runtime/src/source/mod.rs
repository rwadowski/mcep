use crate::DataFrame;
use actix::{Actor, Context};

pub mod kafka;

trait Source {
    fn fetch() -> DataFrame;
}

enum SourceActorMessage {
    Poll,
}

pub struct SourceActor {
    source: Box<dyn Source>,
}

impl Actor for SourceActor {
    type Context = Context<Self>;
}
