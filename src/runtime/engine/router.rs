use std::collections::{HashMap, HashSet};

use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::SinkId;
use crate::types::deployment::source::SourceId;
use crate::types::deployment::BlockId;

pub struct Router {
    block_connections: HashMap<BlockId, HashSet<BlockId>>,
    sink_connections: HashMap<BlockId, HashSet<SinkId>>,
    source_connections: HashMap<SourceId, HashSet<BlockId>>,
}

impl Router {
    pub fn new(connections: &Vec<BlockConnection>) -> Router {
        Router {
            block_connections: build_block_connections(connections),
            sink_connections: build_sink_connections(connections),
            source_connections: build_source_connections(connections),
        }
    }

    pub fn block_targets(&self, source_id: &BlockId) -> HashSet<BlockId> {
        self.block_connections
            .get(&source_id)
            .unwrap_or(&HashSet::new())
            .clone()
    }

    pub fn sink_targets(&self, source_id: &BlockId) -> HashSet<SinkId> {
        self.sink_connections
            .get(source_id)
            .unwrap_or(&HashSet::new())
            .clone()
    }

    pub fn source_targets(&self) -> HashMap<SourceId, HashSet<BlockId>> {
        self.source_connections.clone()
    }
}

fn build_block_connections(
    connections: &Vec<BlockConnection>,
) -> HashMap<BlockId, HashSet<BlockId>> {
    let mut result: HashMap<BlockId, HashSet<BlockId>> = HashMap::new();
    for connection in connections.iter() {
        let key = connection.from.block.clone();
        if let Some(block_id) = key {
            let mut targets: HashSet<BlockId> =
                result.get(&block_id).unwrap_or(&HashSet::new()).clone();
            if let Some(target_id) = &connection.to.block {
                targets.insert(target_id.clone());
            }
            result.insert(block_id, targets);
        }
    }
    result
}

fn build_sink_connections(connections: &Vec<BlockConnection>) -> HashMap<BlockId, HashSet<SinkId>> {
    let mut result: HashMap<BlockId, HashSet<SinkId>> = HashMap::new();
    for connection in connections.iter() {
        let sink_opt = connection.to.sink.clone();
        let block_opt = connection.from.block.clone();
        match (sink_opt, block_opt) {
            (Some(sink_id), Some(block_id)) => {
                let mut sinks = result.get(&block_id).unwrap_or(&HashSet::new()).clone();
                sinks.insert(sink_id);
                result.insert(block_id, sinks);
            }
            _ => {}
        }
    }
    result
}

fn build_source_connections(
    connections: &Vec<BlockConnection>,
) -> HashMap<SourceId, HashSet<BlockId>> {
    let mut result: HashMap<SourceId, HashSet<BlockId>> = HashMap::new();
    for connection in connections.iter() {
        let source_opt = connection.from.source.clone();
        let block_opt = connection.to.block.clone();
        match (source_opt, block_opt) {
            (Some(source_id), Some(block_id)) => {
                let mut source_targets = result.get(&source_id).unwrap_or(&HashSet::new()).clone();
                source_targets.insert(block_id);
                result.insert(source_id, source_targets);
            }
            _ => {}
        }
    }
    result
}
