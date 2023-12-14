use crate::engine::{EngineActor, EngineActorMessage};
use crate::source::Source;
use crate::{util, DataFrame, Message, Origin};
use actix::Addr;
use kafka::client::FetchOffset;
use kafka::consumer::Consumer;
use serde_derive::{Deserialize, Serialize};
use types::config::app::Kafka;
use types::deployment::source::SourceId;

pub struct KafkaSource {
    config: Kafka,
    consumer: Consumer,
}

impl KafkaSource {
    pub fn new(cfg: &Kafka) -> Result<Box<KafkaSource>, String> {
        let consumer = Consumer::from_hosts(cfg.hosts.clone())
            .with_fallback_offset(FetchOffset::Latest)
            .with_topic(cfg.topics.output.clone())
            .with_client_id(cfg.client_id.clone())
            .create()
            .map_err(|e| e.to_string())?;
        Ok(Box::new(KafkaSource {
            config: cfg.clone(),
            consumer,
        }))
    }
}

impl Source for KafkaSource {
    fn fetch(&mut self) -> Result<Vec<DataFrame>, String> {
        let mut result: Vec<DataFrame> = Vec::new();
        for ms in self.consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let origin = Origin::from(self.config.source_id());
                let payload = std::str::from_utf8(m.value).unwrap();
                let decoded =
                    serde_json::from_str::<Message>(payload).map_err(|err| err.to_string())?;
                let df = to_data_frame(origin, decoded);
                result.push(df);
            }
        }
        Ok(result)
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
