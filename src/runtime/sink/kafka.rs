use std::time::Duration;

use async_nats::Client;
use futures::StreamExt;
use log::error;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::ClientConfig;
use tokio::task::JoinHandle;

use crate::runtime::engine::flow::sink_subject;
use crate::runtime::{DataFrame, Message as DataMessage};
use crate::types::config::Kafka;
use crate::types::deployment::sink::SinkId;
use crate::utils;

pub async fn spawn_sink(
    cfg: &Kafka,
    nats: Client,
) -> Result<(SinkId, JoinHandle<()>), String> {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.host_list().join(","))
        .set("client.id", &cfg.client_id)
        .create()
        .map_err(|e| e.to_string())?;

    let sink_id = cfg.sink_id();
    let subject = sink_subject(&sink_id);
    let topic = cfg.topics.output.clone();

    let handle = tokio::spawn(async move {
        run_sink(nats, subject, topic, producer).await;
    });

    Ok((sink_id, handle))
}

async fn run_sink(nats: Client, subject: String, topic: String, producer: BaseProducer) {
    let mut sub = match nats.subscribe(subject.clone()).await {
        Ok(s) => s,
        Err(e) => {
            error!("sink failed to subscribe to '{}': {}", subject, e);
            return;
        }
    };

    while let Some(msg) = sub.next().await {
        let frames: Vec<DataFrame> = match serde_json::from_slice(&msg.payload) {
            Ok(f) => f,
            Err(e) => {
                error!("sink '{}' failed to deserialize frames: {}", subject, e);
                continue;
            }
        };

        match frames_to_record(&frames) {
            Ok(records) => {
                for (key, payload) in records {
                    let record = BaseRecord::to(&topic).key(&key).payload(&payload);
                    if let Err((err, _)) = producer.send(record) {
                        error!("sink failed to send record: {}", err);
                    }
                }
                producer.poll(Duration::from_millis(0));
            }
            Err(e) => error!("sink failed to convert frames to records: {}", e),
        }
    }
}

pub fn frames_to_record(frames: &Vec<DataFrame>) -> Result<Vec<(String, String)>, String> {
    let mut records: Vec<(String, String)> = Vec::new();
    for frame in frames {
        let msg = frame_to_message(frame);
        records.push(message_to_record(&msg)?);
    }
    Ok(records)
}

pub fn messages_to_records(messages: &Vec<DataMessage>) -> Result<Vec<(String, String)>, String> {
    let mut records: Vec<(String, String)> = Vec::new();
    for message in messages {
        records.push(message_to_record(message)?);
    }
    Ok(records)
}

pub fn message_to_record(msg: &DataMessage) -> Result<(String, String), String> {
    let key = msg.key().to_string();
    let value = msg.as_json().map_err(utils::to_string)?;
    Ok((key, value))
}

fn frame_to_message(frame: &DataFrame) -> DataMessage {
    DataMessage {
        ts: frame.ts,
        name: frame.name.clone(),
        value: frame.value.clone(),
    }
}
