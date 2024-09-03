use crate::runtime::sink::kafka::messages_to_records;
use crate::runtime::Message;
use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::{Deployment, DeploymentId};
use crate::utils;
use kafka::producer::Producer;

pub fn send(hosts: Vec<String>, topic: String, messages: Vec<Message>) -> Result<usize, String> {
    let mut producer = Producer::from_hosts(hosts)
        .with_client_id("mcep-tools".to_string())
        .create()
        .map_err(utils::to_string)?;

    let records = messages_to_records(&topic, &messages)?;
    let result = producer.send_all(&records);
    result
        .map(|confirms| confirms.len())
        .map_err(utils::to_string)
}
pub fn create_definition(
    host: String,
    port: i32,
    definition: Definition,
) -> Result<DefinitionId, String> {
    let path = format!("http://{}:{}/api/v1/definition", host, port);
    let body = ureq::post(path.as_str())
        .send_json(ureq::json!(definition))
        .map_err(utils::to_string)?
        .into_string()
        .map_err(utils::to_string)?;
    let result = serde_json::from_str::<Definition>(&body).map_err(utils::to_string)?;
    Ok(result.id)
}

pub fn delete_definition(
    host: String,
    port: i32,
    definition_id: DefinitionId,
) -> Result<(), String> {
    let path = format!(
        "http://{}:{}/api/v1/definition/{}",
        host, port, definition_id
    );
    delete(path)
}

pub fn create_deployment(
    host: String,
    port: i32,
    deployment: Deployment,
) -> Result<DeploymentId, String> {
    let path = format!("http://{}:{}/api/v1/deployment", host, port);
    let body = ureq::post(path.as_str())
        .send_json(ureq::json!(deployment))
        .map_err(utils::to_string)?
        .into_string()
        .map_err(utils::to_string)?;
    let result = serde_json::from_str::<Deployment>(&body).map_err(utils::to_string)?;
    Ok(result.id)
}

pub fn delete_deployment(
    host: String,
    port: i32,
    deployment_id: DeploymentId,
) -> Result<(), String> {
    let path = format!(
        "http://{}:{}/api/v1/deployment/{}",
        host, port, deployment_id
    );
    delete(path)
}

fn delete(path: String) -> Result<(), String> {
    ureq::delete(path.as_str())
        .call()
        .map_err(utils::to_string)?
        .into_string()
        .map_err(utils::to_string)
        .map(|_| ())
}
