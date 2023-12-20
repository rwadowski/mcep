use kafka::client::FetchOffset;
use kafka::consumer::Consumer;
use log::debug;

use types::config::Kafka;

use crate::source::Source;
use crate::{DataFrame, Message, Origin};

pub struct KafkaSource {
    config: Kafka,
    consumer: Consumer,
}

impl KafkaSource {
    pub fn new(cfg: &Kafka) -> Result<Box<KafkaSource>, String> {
        debug!("trying to create kafka source: {:?}", cfg);
        let consumer = Consumer::from_hosts(cfg.host_list())
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
        debug!("trying to fetch data from kafka {:?}", self.config);
        let mut result: Vec<DataFrame> = Vec::new();
        for ms in self.consumer.poll().unwrap().iter() {
            //TODO - handle failure
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
