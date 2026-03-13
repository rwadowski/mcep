use async_nats::Client;
use log::error;
use tokio::task::JoinHandle;

use crate::runtime::source::kafka::KafkaSource;
use crate::runtime::DataFrame;
use crate::types::config::Kafka;

pub mod kafka;

pub trait Source: Send {
    fn fetch(&mut self) -> Result<Vec<DataFrame>, String>;
}

pub fn spawn_source(cfg: &Kafka, nats: Client) -> Result<JoinHandle<()>, String> {
    let source = KafkaSource::new(cfg)?;
    let handle = tokio::spawn(async move {
        run_source(nats, source).await;
    });
    Ok(handle)
}

async fn run_source(nats: Client, mut source: Box<dyn Source>) {
    loop {
        match source.fetch() {
            Ok(frames) => {
                for frame in frames {
                    match serde_json::to_vec(&frame) {
                        Ok(payload) => {
                            if let Err(e) = nats.publish("mcep.frames", payload.into()).await {
                                error!("source failed to publish frame: {}", e);
                            }
                        }
                        Err(e) => error!("source failed to serialize frame: {}", e),
                    }
                }
            }
            Err(e) => error!("source fetch error: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
