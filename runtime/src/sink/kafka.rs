use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use kafka::producer::{Producer, Record};

use types::config::Kafka;
use types::deployment::sink::SinkId;

use crate::{DataFrame, Message as DataMessage};

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
