use crate::types::deployment::{Deployment, NewDeployment, UpdateDeployment};

pub mod create;
mod create_test;
pub mod delete;
mod delete_test;
pub mod get;
mod get_test;
pub mod update;
mod update_test;

pub trait Service {
    async fn create(new_deployment: &NewDeployment) -> Result<Deployment, String>;
    async fn delete(id: i32) -> Result<(), String>;

    async fn get(id: i32) -> Result<Deployment, String>;

    async fn get_all_deployments() -> Result<Vec<Deployment>, String>;

    async fn update(update_deployment: &UpdateDeployment) -> Result<Deployment, String>;
}
