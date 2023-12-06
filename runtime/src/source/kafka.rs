use crate::engine::{EngineActor, EngineActorMessage};
use crate::{util, DataFrame, Message, Origin};
use actix::{Actor, Addr};
use kafka::client::FetchOffset;
use kafka::consumer::Consumer;
use serde_derive::{Deserialize, Serialize};
use types::deployment::source::SourceId;

#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaSourceConfig {
    id: SourceId,
    hosts: Vec<String>,
    topic: String,
    client_id: String,
}

pub struct KafkaSourceActor {}

impl Actor for KafkaSourceActor {
    type Context = Context<Self>;
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
                let origin = Origin::from(config.id.clone());
                let payload = std::str::from_utf8(m.value).unwrap();
                let decoded =
                    serde_json::from_str::<Message>(payload).map_err(|err| err.to_string())?;
                let df = to_data_frame(origin, decoded);
                let _ = addr.send(EngineActorMessage::Process(df));
            }
        }
    }
}

fn to_data_frame(origin: Origin, msg: Message) -> DataFrame {
    DataFrame {
        origin,
        ts: msg.ts,
        name: msg.name,
        value: msg.value,
    }
}
