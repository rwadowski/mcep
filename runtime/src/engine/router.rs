use crate::engine::BlockId;
use std::collections::{HashMap, HashSet};
use types::deployment::connection::BlockConnection;
use types::deployment::DeploymentId;

struct RouterItem {
    connections: HashMap<BlockId, HashSet<BlockId>>,
}
impl From<&Vec<BlockConnection>> for RouterItem {
    fn from(values: &Vec<BlockConnection>) -> Self {
        let mut connections: HashMap<BlockId, HashSet<BlockId>> = HashMap::new();
        for connection in values {
            let key = connection.from.block.clone();
            let mut targets: HashSet<BlockId> =
                connections.get(&key).unwrap_or(&HashSet::new()).clone();
            targets.insert(connection.to.block.clone());
            connections.insert(key, targets);
        }
        RouterItem { connections }
    }
}

pub struct Router {
    deployments: HashMap<DeploymentId, RouterItem>,
    connections: HashMap<BlockId, HashSet<BlockId>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            deployments: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    pub fn add_connections(
        &mut self,
        deployment_id: DeploymentId,
        connections: &Vec<BlockConnection>,
    ) {
        let item = RouterItem::from(connections);
        self.deployments.insert(deployment_id, item);
        self.recalculate_block_to_block_map();
    }

    pub fn remove_connections(&mut self, deployment_id: &DeploymentId) {
        self.deployments.remove(&deployment_id);
        self.recalculate_block_to_block_map();
    }
    fn recalculate_block_to_block_map(&mut self) {
        let updated: HashMap<BlockId, HashSet<BlockId>> = self
            .deployments
            .iter()
            .flat_map(|(_, item)| item.connections.clone())
            .collect();
        self.connections = updated;
    }

    pub fn targets(&self, id: &BlockId) -> HashSet<BlockId> {
        let result = self.connections.get(id);
        match result {
            Some(set) => set.clone(),
            None => HashSet::new(),
        }
    }
}
