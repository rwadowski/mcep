use std::collections::{HashMap, HashSet};
use crate::engine::BlockId;

pub(crate) struct Router {
    table: HashMap<BlockId, HashSet<BlockId>>,
}

impl Router {
    fn targets(&self, id: &BlockId) -> HashSet<BlockId> {
        let result = self.table.get(id);
        match result {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }
}