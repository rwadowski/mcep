use crate::sink::SinkId;
use crate::{util, DataFrame};
use crossbeam_channel::Receiver;
use kafka::producer::{Producer, Record};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaSinkConfig {
    id: SinkId,
    hosts: Vec<String>,
    topic: String,
    client_id: String,
}

pub fn run_kafka_sink(receiver: Receiver<DataFrame>) -> Result<(), String> {
    let config: KafkaSinkConfig = util::load("kafka.sink.toml".to_string())?;
    let mut producer = Producer::from_hosts(config.hosts)
        .with_client_id(config.client_id)
        .create()
        .map_err(|e| e.to_string())?;
    loop {
        let df = receiver.recv().unwrap();
        let payload = Record::from_key_value(config.topic.as_str(), df.key(), df.as_json());
        producer.send(&payload).expect("should end");
    }
}
