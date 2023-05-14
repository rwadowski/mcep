use std::rc::Rc;

use crate::definition::block::Block;
use crate::definition::connection::sink::Sink;
use crate::definition::connection::source::Source;
use crate::definition::connection::Connection;

pub struct App {
    blocks: Vec<Rc<dyn Block>>,
    connections: Vec<Connection>,
    sinks: Vec<Sink>,
    sources: Vec<Source>,
}

impl App {
    fn new() -> App {
        App {
            blocks: Vec::new(),
            connections: Vec::new(),
            sinks: Vec::new(),
            sources: Vec::new(),
        }
    }

    fn add_block(&self, block: Rc<dyn Block>) -> App {
        let mut updated = self.blocks.clone();
        updated.push(block);
        App {
            blocks: updated,
            connections: self.connections.clone(),
            sinks: self.sinks.clone(),
            sources: self.sources.clone(),
        }
    }

    fn add_connection(&self, connection: Connection) -> App {
        let mut updated = self.connections.clone();
        updated.push(connection);
        App {
            blocks: self.blocks.clone(),
            connections: updated,
            sinks: self.sinks.clone(),
            sources: self.sources.clone(),
        }
    }
}
