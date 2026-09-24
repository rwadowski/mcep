use crate::database::{DefinitionStore, DeploymentStore};
use crate::runtime::engine::Engine;
use crate::types::deployment::{Deployment, DeploymentId, NewDeployment, UpdateDeployment};
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;

pub mod create;
mod create_test;
pub mod delete;
mod delete_test;
pub mod get;
mod get_test;
pub mod update;
mod update_test;

pub trait DeploymentService {
    async fn create(&self, new_deployment: &NewDeployment) -> Result<Deployment, String>;

    async fn delete(&self, id: DeploymentId) -> Result<(), String>;

    async fn get(&self, id: DeploymentId) -> Result<Deployment, String>;

    async fn get_all(&self) -> Result<Vec<Deployment>, String>;

    async fn update(&self, update: &UpdateDeployment) -> Result<Deployment, String>;
}

pub struct Service<DeplStore: DeploymentStore + Clone, DefStore: DefinitionStore + Clone> {
    store: DeplStore,
    definitions: DefStore,
    engine: Arc<Engine>,
}

impl<DeplStore: DeploymentStore + Clone, DefStore: DefinitionStore + Clone>
    Service<DeplStore, DefStore>
{
    pub fn new(store: DeplStore, definitions: DefStore, engine: Arc<Engine>) -> Self {
        Service {
            store,
            definitions,
            engine,
        }
    }
}

impl<DeplStore: DeploymentStore + Clone, DefStore: DefinitionStore + Clone> Clone
    for Service<DeplStore, DefStore>
{
    fn clone(&self) -> Self {
        Service {
            store: self.store.clone(),
            definitions: self.definitions.clone(),
            engine: self.engine.clone(),
        }
    }
}

impl<DeplStore: DeploymentStore + Clone, DefStore: DefinitionStore + Clone> DeploymentService
    for Service<DeplStore, DefStore>
{
    async fn create(&self, new_deployment: &NewDeployment) -> Result<Deployment, String> {
        let deployment = self.store.create_deployment(new_deployment).await?;

        let definition_ids: Vec<_> = deployment.definition_ids().into_iter().collect();
        let definitions = self.definitions.get_definitions(&definition_ids).await?;
        let definitions_map: HashMap<_, _> = definitions.into_iter().map(|d| (d.id, d)).collect();

        match self.engine.deploy(&deployment, &definitions_map).await {
            Ok(()) => {
                info!("deployment {} deployed", deployment.id);
                Ok(deployment)
            }
            Err(err) => {
                error!("deployment {} not deployed: {}", deployment.id, err);
                Err(err)
            }
        }
    }

    async fn delete(&self, id: DeploymentId) -> Result<(), String> {
        let deployment = self.store.get_deployment(id).await?;
        self.engine.undeploy(&deployment).await;
        self.store.delete_deployment(id).await
    }

    async fn get(&self, id: DeploymentId) -> Result<Deployment, String> {
        self.store.get_deployment(id).await
    }

    async fn get_all(&self) -> Result<Vec<Deployment>, String> {
        self.store.get_all_deployments().await
    }

    async fn update(&self, update: &UpdateDeployment) -> Result<Deployment, String> {
        self.store.update_deployment(update).await
    }
}
