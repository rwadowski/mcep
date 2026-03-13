use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use async_nats::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::{Deployment, DeploymentId};

use crate::runtime::engine::flow::spawn_flow;

mod block;
pub mod flow;
pub mod router;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Boolean(bool),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
    Text(String),
    Array(Vec<Data>),
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        return &self;
    }
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Data::Boolean(b) => b.hash(state),
            Data::UnsignedInt(i) => i.hash(state),
            Data::SignedInt(i) => i.hash(state),
            Data::Text(s) => s.hash(state),
            Data::Float(f) => f.to_string().hash(state),
            Data::Array(arr) => arr.hash(state),
        }
    }
}

pub struct Engine {
    nats: Client,
    deployments: Mutex<HashMap<DeploymentId, Vec<JoinHandle<()>>>>,
}

impl Engine {
    pub fn new(nats: Client) -> Arc<Engine> {
        Arc::new(Engine {
            nats,
            deployments: Mutex::new(HashMap::new()),
        })
    }

    pub async fn deploy(
        &self,
        deployment: &Deployment,
        definitions: &HashMap<DefinitionId, Definition>,
    ) -> Result<(), String> {
        let handles = spawn_flow(&self.nats, deployment, definitions).await?;
        self.deployments.lock().await.insert(deployment.id, handles);
        Ok(())
    }

    pub async fn undeploy(&self, deployment: &Deployment) {
        if let Some(handles) = self.deployments.lock().await.remove(&deployment.id) {
            for handle in handles {
                handle.abort();
            }
        }
    }
}
