use crate::engine::BlockId;
use std::collections::{HashMap, HashSet};
use types::deployment::connection::BlockConnection;

pub struct Router {
    table: HashMap<BlockId, HashSet<BlockId>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            table: HashMap::new(),
        }
    }

    pub fn update(&mut self, connections: &Vec<BlockConnection>) {
        for connection in connections {
            let key = connection.from.block.clone();
            let mut targets: HashSet<BlockId> =
                self.table.get(&key).unwrap_or(&HashSet::new()).clone();
            targets.insert(connection.to.block.clone());
            self.table.insert(key, targets);
        }
    }
    pub fn targets(&self, id: &BlockId) -> HashSet<BlockId> {
        let result = self.table.get(id);
        match result {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }
}
