use crossbeam_channel::Sender;
use rocket::serde::Deserialize;
use types::deployment::{Command, Deployment};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDeployment {
    pub name: String,
    pub version: String,
    pub application_id: i32,
}
pub async fn create_deployment(sender: &Sender<Command>, new_deployment: NewDeployment) -> Option<Deployment> {

    None
}