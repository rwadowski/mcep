use std::collections::{HashMap, HashSet};

use async_nats::Client;
use futures::StreamExt;
use log::debug;
use tokio::task::JoinHandle;

use crate::runtime::engine::block::{new_block, spawn_block};
use crate::runtime::engine::router::Router;
use crate::runtime::DataFrame;
use crate::types::definition::block::new_block_from_str;
use crate::types::definition::block::Block as BlockDefinition;
use crate::types::definition::{Definition, DefinitionId};
use crate::types::deployment::sink::SinkId;
use crate::types::deployment::source::SourceId;
use crate::types::deployment::{BlockId, Deployment, DeploymentId};

pub fn block_subject(deployment_id: DeploymentId, block_id: &BlockId) -> String {
    format!(
        "mcep.block.{}.{}.{}",
        deployment_id, block_id.definition_id, block_id.id
    )
}

pub fn sink_subject(sink_id: &SinkId) -> String {
    format!("mcep.sink.{}", sink_id.value)
}

pub async fn spawn_flow(
    nats: &Client,
    deployment: &Deployment,
    definitions: &HashMap<DefinitionId, Definition>,
) -> Result<Vec<JoinHandle<()>>, String> {
    let router = Router::new(&deployment.connections);
    let deployment_id = deployment.id;
    let mut handles = Vec::new();

    for deployed_block in deployment.blocks.iter() {
        let definition = definitions
            .get(&deployed_block.definition_id)
            .ok_or("no definition provided")?;
        let body = definition.body.to_string();
        let block_definition: Box<dyn BlockDefinition> = new_block_from_str(body.as_str())?;
        let block = new_block(block_definition, deployment.id, deployed_block.id)?;
        let block_id = deployed_block.id();

        let target_block_subjects: Vec<String> = router
            .block_targets(&block_id)
            .iter()
            .map(|bid| block_subject(deployment_id, bid))
            .collect();
        let target_sink_subjects: Vec<String> = router
            .sink_targets(&block_id)
            .iter()
            .map(sink_subject)
            .collect();

        let handle = spawn_block(
            nats.clone(),
            deployment_id,
            block,
            target_block_subjects,
            target_sink_subjects,
        );
        handles.push(handle);
    }

    let sources = router.source_targets();
    let deployment_name = deployment.name.clone();
    let nats_clone = nats.clone();
    let flow_handle = tokio::spawn(async move {
        run_flow(nats_clone, deployment_id, deployment_name, sources).await;
    });
    handles.push(flow_handle);

    Ok(handles)
}

async fn run_flow(
    nats: Client,
    deployment_id: DeploymentId,
    deployment_name: String,
    sources: HashMap<SourceId, HashSet<BlockId>>,
) {
    let mut sub = match nats.subscribe("mcep.frames").await {
        Ok(s) => s,
        Err(e) => {
            log::error!(
                "flow '{}' failed to subscribe to mcep.frames: {}",
                deployment_name,
                e
            );
            return;
        }
    };

    while let Some(msg) = sub.next().await {
        let df: DataFrame = match serde_json::from_slice(&msg.payload) {
            Ok(df) => df,
            Err(e) => {
                log::error!("flow '{}' failed to deserialize DataFrame: {}", deployment_name, e);
                continue;
            }
        };

        debug!("flow '{}' processing {:?}", deployment_name, df);

        if let Some(source_id) = &df.origin.source {
            if let Some(block_ids) = sources.get(source_id) {
                for block_id in block_ids {
                    let subject = block_subject(deployment_id, block_id);
                    let payload = match serde_json::to_vec(&vec![df.clone()]) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("flow '{}' failed to serialize frame: {}", deployment_name, e);
                            continue;
                        }
                    };
                    if let Err(e) = nats.publish(subject.clone(), payload.into()).await {
                        log::error!(
                            "flow '{}' failed to publish to block {}: {}",
                            deployment_name,
                            subject,
                            e
                        );
                    }
                }
            }
        }
    }
}
