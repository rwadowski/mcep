use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::definition::{Application, ApplicationId};

#[derive(Debug)]
pub enum Command {
    CreateApplication(Application),
    DeleteApplication(ApplicationId)
}

#[derive(Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, FromRow)]
pub struct Deployment {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub application_id: i32,
}