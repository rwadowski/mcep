use std::collections::{HashMap, HashSet};

use types::deployment::connection::BlockConnection;
use types::deployment::BlockId;

pub struct Router {
    connections: HashMap<BlockId, HashSet<BlockId>>,
}

impl Router {
    pub fn new(connections: &Vec<BlockConnection>) -> Router {
        Router {
            connections: build_connections(connections),
        }
    }

    pub fn targets(&self, source_id: &BlockId) -> HashSet<BlockId> {
        let result = self.connections.get(source_id);
        match result {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }
}

fn build_connections(connections: &Vec<BlockConnection>) -> HashMap<BlockId, HashSet<BlockId>> {
    let mut result: HashMap<BlockId, HashSet<BlockId>> = HashMap::new();
    for connection in connections {
        let key = connection.from.block.clone();
        let mut targets: HashSet<BlockId> = result.get(&key).unwrap_or(&HashSet::new()).clone();
        targets.insert(connection.to.block.clone());
        result.insert(key, targets);
    }
    result
}
