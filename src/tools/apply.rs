use crate::runtime::Message;
use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::{Deployment, DeploymentId};
use crate::utils;
use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use rdkafka::ClientConfig;
use std::time::Duration;

pub fn send(hosts: Vec<String>, topic: String, messages: Vec<Message>) -> Result<usize, String> {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &hosts.join(","))
        .set("client.id", "mcep-tools")
        .create()
        .map_err(utils::to_string)?;

    let mut sent = 0;

    for message in messages {
        let payload = serde_json::to_string(&message).map_err(utils::to_string)?;
        let record: BaseRecord<String, String> = BaseRecord::to(&topic).payload(&payload);
        match producer.send(record) {
            Ok(_) => sent += 1,
            Err((KafkaError::MessageProduction(_retryable), _)) => {
                return Err("retryable Kafka error".to_string());
            }
            Err((err, _)) => {
                return Err(utils::to_string(&err));
            }
        }
    }

    producer
        .flush(Duration::from_secs(1))
        .map_err(utils::to_string)?;
    Ok(sent)
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
