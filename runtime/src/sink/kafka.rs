use crate::{util, DataFrame, Message as DataMessage};
use actix::{Actor, Addr, Context, Handler, Message};
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
                        let msg = frame_to_message(&frame);
                        let key = msg.key();
                        let value = msg.as_json().expect("msg should be serialized");
                        Record::from_key_value(self.topic.as_str(), key, value)
                    })
                    .collect();
                let _ = self.producer.send_all(&records); // todo - evaluate result
            }
        }
    }
}

fn frame_to_message(frame: &DataFrame) -> DataMessage {
    DataMessage {
        ts: frame.ts,
        name: frame.name.clone(),
        value: frame.value.clone(),
    }
}
