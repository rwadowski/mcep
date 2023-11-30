use crate::engine::block::{new_block, Block};
use crate::engine::router::Router;
use crate::DataFrame;
use crossbeam_channel::{select, Receiver, Sender};
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use types::definition::block::new_block_from_str;
use types::definition::block::Block as BlockDefinition;
use types::definition::{Definition, DefinitionId};
use types::deployment::connection::junction::{BlockJunction, DefinitionJunction};
use types::deployment::{BlockId, Command, Deployment};

mod block;
mod router;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Boolean(bool),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
    Text(String),
    Array(Vec<Data>),
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        return &self;
    }
}
impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Data::Boolean(b) => b.hash(state),
            Data::UnsignedInt(i) => i.hash(state),
            Data::SignedInt(i) => i.hash(state),
            Data::Text(s) => s.hash(state),
            Data::Float(f) => f.to_string().hash(state),
            Data::Array(arr) => arr.hash(state),
        }
    }
}

pub struct Engine {
    blocks: HashMap<BlockId, Box<dyn Block>>,
    connections: HashMap<BlockJunction, Vec<BlockJunction>>,
    command_rx: Receiver<Command>,
    data_input: Receiver<DataFrame>,
    data_output: Sender<DataFrame>,
}

impl Engine {
    pub fn new(
        command_rx: Receiver<Command>,
        data_input: Receiver<DataFrame>,
        data_output: Sender<DataFrame>,
    ) -> Engine {
        Engine {
            blocks: HashMap::new(),
            connections: HashMap::new(),
            command_rx,
            data_input,
            data_output,
        }
    }
}
pub fn run(
    command_rx: Receiver<Command>,
    data_input: Receiver<DataFrame>,
    data_output: Sender<DataFrame>,
) {
    let engine: &mut Engine = &mut Engine::new(command_rx, data_input, data_output);
    let router: &mut Router = &mut Router::new();
    loop {
        select! {
            recv(engine.command_rx) -> cmd => {
                cmd
                    .map_err(|err| err.to_string())
                    .iter()
                    .flat_map(|command| process_command(engine, router, command))
                    .collect()
            }
            recv(engine.data_input) -> data_opt => {
                let result: Vec<DataFrame> = data_opt
                    .map_err(|err| err.to_string())
                    .iter()
                    .flat_map(process_data)
                    .collect();
                let sent: Result<(), String> = result.into_iter().map(|frame| {
                    engine.data_output.send(frame).map_err(|err| err.to_string())
                }).collect();
                sent.unwrap()
            }
        }
    }
}

fn process_command(
    engine: &mut Engine,
    router: &mut Router,
    command: &Command,
) -> Result<(), String> {
    debug!("command {:?} received", command);
    match command {
        Command::Deploy(deployment, definitions) => {
            deploy_blocks(engine, router, &deployment, &definitions)
        }
        Command::Undeploy(deployment) => undeploy_blocks(engine, router, deployment),
    }
}

//TODO - update connections / router - decide whether router is needed
fn deploy_blocks(
    engine: &mut Engine,
    router: &mut Router,
    deployment: &Deployment,
    definitions: &Vec<Definition>,
) -> Result<(), String> {
    for definition in definitions.iter() {
        let block_definition: Box<dyn BlockDefinition> =
            new_block_from_str(definition.body.to_string().as_str())?;
        let block = new_block(deployment.id, block_definition)?;
        engine.blocks.insert(block.id(), block);
        router.update(&deployment.connections);
    }
    Ok(())
}

//TODO - implement me
fn undeploy_blocks(
    engine: &mut Engine,
    router: &mut Router,
    deployment: &Deployment,
) -> Result<(), String> {
    Ok(())
}

//TODO - implement me
fn process_data(data: &DataFrame) -> Result<DataFrame, String> {
    Err("not_implemented".to_string())
}
