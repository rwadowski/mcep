use crate::{util, DataFrame};
use actix::{Actor, Addr, Context, Handler, Message};
use crossbeam_channel::Receiver;
use kafka::producer::{Producer, Record};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use types::deployment::sink::SinkId;

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
    pub fn new() -> Result<HashMap<SinkId, Addr<KafkaSinkActor>>, String> {
        let config: KafkaSinkConfig = util::load("kafka.sink.toml".to_string())?;
        let producer = Producer::from_hosts(config.hosts)
            .with_client_id(config.client_id)
            .create()
            .map_err(|e| e.to_string())?;
        let actor = KafkaSinkActor {
            topic: config.topic,
            producer,
        };
        let as_map: HashMap<SinkId, Addr<KafkaSinkActor>> =
            HashMap::from([(config.id, actor.start())]);
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
                let records: Vec<Record<String, String>> = frames
                    .iter()
                    .map(|frame| {
                        Record::from_key_value(self.topic.as_str(), frame.key(), frame.as_json())
                    })
                    .collect();
                let _ = self.producer.send_all(&records); // todo - evaluate result
            }
        }
    }
}
