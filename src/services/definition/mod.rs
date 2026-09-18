use crate::types::definition::{Definition, NewDefinition, UpdateDefinition};
use std::collections::HashSet;

pub mod create;
mod create_test;
pub mod delete;
mod delete_test;
pub mod get;
mod get_test;
pub mod update;
mod update_test;

pub trait Service {
    async fn create(new_definition: NewDefinition) -> Result<Definition, String>;
    async fn delete(id: i64) -> Result<(), String>;

    async fn get(id: i64) -> Result<Definition, String>;

    async fn get_list(ids: HashSet<i64>) -> Result<Vec<Definition>, String>;

    async fn update(update: UpdateDefinition) -> Result<Definition, String>;
}
