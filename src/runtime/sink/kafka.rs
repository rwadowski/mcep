use actix::{Actor, Addr, Context, Handler, Message};
use log::error;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::ClientConfig;
use std::collections::HashMap;
use std::time::Duration;

use crate::types::config::Kafka;
use crate::types::deployment::sink::SinkId;

use crate::runtime::{DataFrame, Message as DataMessage};
use crate::utils;

#[derive(Message)]
#[rtype(result = "()")]
pub enum KafkaSinkActorMessage {
    Send(Vec<DataFrame>),
}

pub struct KafkaSinkActor {
    topic: String,
    producer: BaseProducer,
}

impl KafkaSinkActor {
    pub fn new(cfg: &Kafka) -> Result<HashMap<SinkId, Addr<KafkaSinkActor>>, String> {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.host_list().join(","))
            .set("client.id", &cfg.client_id)
            .create()
            .map_err(|e| e.to_string())?;
        let actor = KafkaSinkActor {
            topic: cfg.topics.output.clone(),
            producer,
        };
        let as_map: HashMap<SinkId, Addr<KafkaSinkActor>> =
            HashMap::from([(cfg.sink_id(), actor.start())]);
        Ok(as_map)
    }
}

impl Actor for KafkaSinkActor {
    type Context = Context<Self>;
}

impl Handler<KafkaSinkActorMessage> for KafkaSinkActor {
    type Result = ();

    fn handle(&mut self, msg: KafkaSinkActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            KafkaSinkActorMessage::Send(frames) => {
                let records: Vec<(String, String)> =
                    frames_to_record(&frames).expect("convert frames"); // todo - error handling
                for (key, payload) in records {
                    let record = BaseRecord::to(&self.topic).key(&key).payload(&payload);
                    if let Err((err, _record)) = self.producer.send(record) {
                        error!("failed to send record {}", err);
                    }
                }
                self.producer.poll(Duration::from_millis(0));
            }
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

pub fn messages_to_records<'a>(
    messages: &Vec<DataMessage>,
) -> Result<Vec<(String, String)>, String> {
    let mut records: Vec<(String, String)> = Vec::new();
    for message in messages {
        records.push(message_to_record(&message)?);
    }
    Ok(records)
}

pub fn message_to_record<'a>(msg: &DataMessage) -> Result<(String, String), String> {
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
