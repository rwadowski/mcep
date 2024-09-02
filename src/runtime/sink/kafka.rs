use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use kafka::producer::{Producer, Record};

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
    producer: Producer,
}

impl KafkaSinkActor {
    pub fn new(cfg: &Kafka) -> Result<HashMap<SinkId, Addr<KafkaSinkActor>>, String> {
        let producer = Producer::from_hosts(cfg.host_list())
            .with_client_id(cfg.client_id.clone())
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
                let records: Vec<Record<String, String>> =
                    frames_to_record(&self.topic, &frames).expect("convert frames"); // todo - error handling
                let _ = self.producer.send_all(&records); // todo - evaluate result
            }
        }
    }
}

pub fn frames_to_record<'a>(
    topic: &'a String,
    frames: &Vec<DataFrame>,
) -> Result<Vec<Record<'a, String, String>>, String> {
    let mut records: Vec<Record<'a, String, String>> = Vec::new();
    for frame in frames {
        let msg = frame_to_message(frame);
        records.push(message_to_record(topic, &msg)?);
    }
    Ok(records)
}

pub fn messages_to_records<'a>(
    topic: &'a String,
    messages: &Vec<DataMessage>,
) -> Result<Vec<Record<'a, String, String>>, String> {
    let mut records: Vec<Record<'a, String, String>> = Vec::new();
    for message in messages {
        let record = message_to_record(topic, &message)?;
        records.push(record);
    }
    Ok(records)
}

pub fn message_to_record<'a>(
    topic: &'a String,
    msg: &DataMessage,
) -> Result<Record<'a, String, String>, String> {
    let key = msg.key();
    let value = msg.as_json().map_err(utils::to_string)?;
    Ok(Record::from_key_value(topic, key, value))
}

fn frame_to_message(frame: &DataFrame) -> DataMessage {
    DataMessage {
        ts: frame.ts,
        name: frame.name.clone(),
        value: frame.value.clone(),
    }
}
