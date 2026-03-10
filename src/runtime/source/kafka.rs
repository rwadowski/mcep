use crate::runtime::source::Source;
use crate::runtime::{DataFrame, Message, Origin};
use crate::types::config::Kafka;
use log::error;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::{ClientConfig, Message as KafkaMessage};
use std::time::Duration;

pub struct KafkaSource {
    config: Kafka,
    consumer: Option<BaseConsumer>,
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
        let msg = consumer.poll(Duration::from_millis(100));
        let mut frames = vec![];
        if let Some(result) = msg {
            match result {
                Ok(m) => {
                    if let Some(payload) = m.payload() {
                        // Convert payload to DataFrame (your function)
                        let frame = process_message(&self.config, payload)?;
                        frames.push(frame);
                    }
                }
                Err(e) => {
                    return Err(format!("Kafka poll error: {}", e));
                }
            }
        }

        Ok(frames)
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

fn init_consumer(cfg: &Kafka) -> Result<BaseConsumer, String> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.host_list().join(","))
        .set("group.id", &cfg.client_id)
        .set("client.id", &cfg.client_id)
        .set("auto.offset.reset", "latest") // Matches `FetchOffset::Latest`
        .create()
        .map_err(|e| format!("Failed to create Kafka consumer: {}", e))?;

    consumer
        .subscribe(&[&cfg.topics.input])
        .map_err(|e| format!("Failed to subscribe to topic: {}", e))?;

    Ok(consumer)
}

fn process_message(cfg: &Kafka, message: &[u8]) -> Result<DataFrame, String> {
    let origin = Origin::from(cfg.source_id());
    let payload_str = std::str::from_utf8(message).map_err(|e| e.to_string())?;
    let decoded: Message = serde_json::from_str(payload_str).map_err(|e| e.to_string())?;
    Ok(to_data_frame(origin, decoded))
}
