use crate::database::DefinitionStore;
use crate::types::definition::{Definition, DefinitionId, NewDefinition, UpdateDefinition};

pub mod create;
mod create_test;
pub mod delete;
mod delete_test;
pub mod get;
mod get_test;
pub mod update;
mod update_test;

pub trait DefinitionService {
    async fn create(&self, new_definition: &NewDefinition) -> Result<Definition, String>;
    async fn delete(&self, id: DefinitionId) -> Result<(), String>;

    async fn get(&self, id: DefinitionId) -> Result<Definition, String>;

    async fn get_list(&self, ids: &Vec<DefinitionId>) -> Result<Vec<Definition>, String>;

    async fn get_all(&self) -> Result<Vec<Definition>, String>;

    async fn update(&self, update: &UpdateDefinition) -> Result<Definition, String>;
}

pub struct Service<Store: DefinitionStore + Clone> {
    store: Store,
}

impl<Store: DefinitionStore + Clone> Service<Store> {
    pub fn new(store: Store) -> Self {
        Service { store }
    }
}

impl<Store: DefinitionStore + Clone> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Service::new(self.store.clone())
    }
}

impl<Store: DefinitionStore + Clone> DefinitionService for Service<Store> {
    async fn create(&self, new_definition: &NewDefinition) -> Result<Definition, String> {
        self.store.create_definition(new_definition).await
    }

    async fn delete(&self, id: DefinitionId) -> Result<(), String> {
        self.store.delete_definition(id).await
    }

    async fn get(&self, id: DefinitionId) -> Result<Definition, String> {
        self.store.get_definition(id).await
    }

    async fn get_list(&self, ids: &Vec<DefinitionId>) -> Result<Vec<Definition>, String> {
        self.store.get_definitions(ids).await
    }

    async fn get_all(&self) -> Result<Vec<Definition>, String> {
        self.store.get_all_definitions().await
    }

    async fn update(&self, update: &UpdateDefinition) -> Result<Definition, String> {
        self.store.update_definition(update).await
    }
}
