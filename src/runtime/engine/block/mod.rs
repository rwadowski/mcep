use std::collections::HashMap;

use async_nats::Client;
use futures::StreamExt;
use log::{debug, error};
use tokio::task::JoinHandle;

use crate::runtime::engine::block::code::PythonCodeBlock;
use crate::runtime::engine::flow::block_subject;
use crate::runtime::engine::Data;
use crate::runtime::{DataFrame, Name};
use crate::types::definition::block::code::CodeBlock as CodeBlockDefinition;
use crate::types::definition::block::github::Github;
use crate::types::definition::block::{Block as BlockDefinition, BlockType};
use crate::types::deployment::{BlockId, BlockInstanceId, DeploymentId};

pub mod code;
mod mod_test;

pub trait Block: Send {
    fn id(&self) -> BlockId;
    fn run(&mut self, df: &HashMap<Name, Data>) -> Result<Vec<DataFrame>, String>;
}

pub fn new_block(
    definition: Box<dyn BlockDefinition>,
    deployment_id: DeploymentId,
    id: BlockInstanceId,
) -> Result<Box<dyn Block>, String> {
    let block_type = definition.block_type();
    match block_type {
        BlockType::CodeBlock => {
            let def = as_type::<CodeBlockDefinition>(definition)?;
            Ok(Box::new(PythonCodeBlock::new(
                def.source,
                deployment_id,
                id,
                def.inputs,
                def.dependencies,
            )))
        }
        BlockType::Github => {
            let def = as_type::<Github>(definition)?;
            let src = def.source()?;
            Ok(Box::new(PythonCodeBlock::new(
                src,
                deployment_id,
                id,
                def.inputs,
                def.dependencies,
            )))
        }
    }
}

fn as_type<T: Clone + 'static>(definition: Box<dyn BlockDefinition>) -> Result<T, String> {
    match definition.as_any().downcast_ref::<T>() {
        Some(def) => Ok(def.clone()),
        None => Err("can't cast to code block definition".to_string()),
    }
}

pub fn spawn_block(
    nats: Client,
    deployment_id: DeploymentId,
    block: Box<dyn Block>,
    target_block_subjects: Vec<String>,
    target_sink_subjects: Vec<String>,
) -> JoinHandle<()> {
    let subject = block_subject(deployment_id, &block.id());
    tokio::spawn(async move {
        run_block(nats, subject, block, target_block_subjects, target_sink_subjects).await;
    })
}

async fn run_block(
    nats: Client,
    subject: String,
    mut block: Box<dyn Block>,
    target_block_subjects: Vec<String>,
    target_sink_subjects: Vec<String>,
) {
    let mut sub = match nats.subscribe(subject.clone()).await {
        Ok(s) => s,
        Err(e) => {
            error!("block failed to subscribe to '{}': {}", subject, e);
            return;
        }
    };

    let mut state: HashMap<Name, Data> = HashMap::new();

    while let Some(msg) = sub.next().await {
        let frames: Vec<DataFrame> = match serde_json::from_slice(&msg.payload) {
            Ok(f) => f,
            Err(e) => {
                error!("block '{}' failed to deserialize frames: {}", subject, e);
                continue;
            }
        };

        for frame in frames {
            state.insert(frame.name.clone(), frame.value.clone());
        }

        match block.run(&state) {
            Ok(output_frames) => {
                debug!(
                    "block '{}' produced {} output frames",
                    subject,
                    output_frames.len()
                );
                for target in &target_block_subjects {
                    publish_frames(&nats, target, &output_frames).await;
                }
                for target in &target_sink_subjects {
                    publish_frames(&nats, target, &output_frames).await;
                }
            }
            Err(e) => error!("block '{}' failed to process message: {}", subject, e),
        }
    }
}

async fn publish_frames(nats: &Client, subject: &str, frames: &Vec<DataFrame>) {
    match serde_json::to_vec(frames) {
        Ok(payload) => {
            if let Err(e) = nats.publish(subject.to_string(), payload.into()).await {
                error!("failed to publish frames to '{}': {}", subject, e);
            }
        }
        Err(e) => error!("failed to serialize frames for '{}': {}", subject, e),
    }
}
