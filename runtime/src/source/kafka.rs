use std::time::Instant;

use actix::prelude::*;
use actix::{Actor, Addr};
use crossbeam_channel::Sender;
use kafka::client::FetchOffset;
use kafka::consumer::Consumer;
use serde_derive::{Deserialize, Serialize};

use types::deployment::BlockId;

use crate::engine::Data;
use crate::engine::{EngineActor, EngineActorMessage};
use crate::source::SourceId;
use crate::{util, DataFrame, Name};

#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaSourceConfig {
    id: SourceId,
    hosts: Vec<String>,
    topic: String,
    client_id: String,
}

//TODO - remove me
pub fn run_kafka_source(sender: Sender<DataFrame>) -> Result<(), String> {
    let config: KafkaSourceConfig = util::load("kafka.source.toml".to_string())?;
    let mut consumer = Consumer::from_hosts(config.hosts)
        .with_fallback_offset(FetchOffset::Latest)
        .with_topic(config.topic)
        .with_client_id(config.client_id)
        .create()
        .map_err(|e| e.to_string())?;
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let origin = BlockId {
                    value: config.id.0.clone(), //TODO - fix me
                };
                let payload = std::str::from_utf8(m.value).unwrap().to_string();
                //TODO - fix me - recognize which message is which
                let name = Name::from("fix_me".to_string());
                let df = DataFrame::new(origin, Instant::now(), name, Data::Text(payload));
                sender.send(df).expect("It should be sent");
            }
        }
    }
}

pub fn run_kafka_actor_source(addr: Addr<EngineActor>) -> Result<(), String> {
    let config: KafkaSourceConfig = util::load("kafka.source.toml".to_string())?;
    let mut consumer = Consumer::from_hosts(config.hosts)
        .with_fallback_offset(FetchOffset::Latest)
        .with_topic(config.topic)
        .with_client_id(config.client_id)
        .create()
        .map_err(|e| e.to_string())?;
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let origin = BlockId {
                    value: config.id.0.clone(), //TODO - fix me
                };
                let payload = std::str::from_utf8(m.value).unwrap().to_string();
                //TODO - fix me - recognize which message is which
                let name = Name::from("fix_me".to_string());
                let df = DataFrame::new(origin, Instant::now(), name, Data::Text(payload));
                let _ = addr.send(EngineActorMessage::Process(df));
            }
        }
    }
}
