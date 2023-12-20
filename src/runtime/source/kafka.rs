use crate::runtime::source::Source;
use crate::runtime::{DataFrame, Message, Origin};
use crate::types::config::Kafka;
use kafka::client::FetchOffset;
use kafka::consumer::{Consumer, MessageSets};
use log::error;

pub struct KafkaSource {
    config: Kafka,
    consumer: Option<Consumer>,
}

impl KafkaSource {
    pub fn new(cfg: &Kafka) -> Result<Box<KafkaSource>, String> {
        let mut source = KafkaSource {
            config: cfg.clone(),
            consumer: None,
        };
        source.reinit_consumer();
        Ok(Box::new(source))
    }

    fn reinit_consumer(&mut self) {
        match init_consumer(&self.config) {
            Ok(c) => {
                self.consumer = Some(c);
            }
            Err(err) => {
                error!("failed to reinit kafka consumer {}", err);
                self.consumer = None;
            }
        }
    }

    fn poll(&mut self) -> Result<Vec<DataFrame>, String> {
        let consumer = self
            .consumer
            .as_mut()
            .ok_or("kafka is not initialized".to_string())?;
        let records = consumer.poll().map_err(|err| err.to_string())?;
        sets_to_frames(&self.config, &records)
    }
}

impl Source for KafkaSource {
    fn fetch(&mut self) -> Result<Vec<DataFrame>, String> {
        let res = self.poll();
        if res.is_err() {
            self.reinit_consumer();
        }
        res
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

fn init_consumer(cfg: &Kafka) -> Result<Consumer, String> {
    Consumer::from_hosts(cfg.host_list())
        .with_fallback_offset(FetchOffset::Latest)
        .with_topic(cfg.topics.output.clone())
        .with_client_id(cfg.client_id.clone())
        .create()
        .map_err(|e| e.to_string())
}

fn sets_to_frames(cfg: &Kafka, sets: &MessageSets) -> Result<Vec<DataFrame>, String> {
    let mut result = Vec::new();
    for set in sets.iter() {
        for m in set.messages() {
            let origin = Origin::from(cfg.source_id());
            let payload = std::str::from_utf8(m.value).unwrap();
            let decoded =
                serde_json::from_str::<Message>(payload).map_err(|err| err.to_string())?;
            let df = to_data_frame(origin, decoded);
            result.push(df);
        }
    }
    Ok(result)
}
