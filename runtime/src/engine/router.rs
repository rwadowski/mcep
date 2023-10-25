use std::collections::{HashMap, HashSet};
use crate::engine::BlockId;

pub struct Router {
    table: HashMap<BlockId, HashSet<BlockId>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            table: HashMap::new(),
        }
    }

    pub fn update(&self) {
        //TODO - implement me
    }
    pub fn targets(&self, id: &BlockId) -> HashSet<BlockId> {
        let result = self.table.get(id);
        match result {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }
}