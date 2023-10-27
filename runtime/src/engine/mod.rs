use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use serde_derive::{Deserialize, Serialize};
use crossbeam_channel::{Receiver, Sender, select};
use log::debug;
use types::definition::block::new_block_from_str;
use types::definition::Definition;
use types::deployment::{BlockId, Command, Deployment};
use types::deployment::connection::junction::{BlockJunction, DefinitionJunction};
use crate::DataFrame;
use types::definition::block::{Block as BlockDefinition};
use crate::engine::block::{Block, new_block};
use crate::engine::router::Router;

mod router;
mod block;

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
        return &self
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
    pub fn new(command_rx: Receiver<Command>,
               data_input: Receiver<DataFrame>,
               data_output: Sender<DataFrame>) -> Engine {
        Engine {
            blocks: HashMap::new(),
            connections: HashMap::new(),
            command_rx,
            data_input,
            data_output,
        }
    }
}
pub fn run(command_rx: Receiver<Command>,
           data_input:Receiver<DataFrame>,
           data_output: Sender<DataFrame>){
    let engine = Engine::new(
        command_rx,
        data_input,
        data_output,
    );
    let mut router = Router::new();
    loop {
        select! {
            recv(engine.command_rx) -> cmd => println!("{:?}", cmd.unwrap())
        }
    }
}

async fn process_command(engine: &mut Engine, command: Command) -> Result<(), String> {
    debug!("command {:?} received", command);
    match command {
        Command::Deploy(deployment, definitions) => {
            // let code_block = PythonCodeBlock::new(deployment.id, )
        }
        Command::Undeploy(deploymnent_id) => {}
    }
    Ok(())
}

//TODO - update connections / router - decide whether router is needed
async fn deploy_blocks(engine: &mut Engine,
                       deployment: &Deployment,
                       definitions: &Vec<Definition>) -> Result<(), String> {
    for definition in definitions.iter() {
        let block_definition: Box<dyn BlockDefinition> = new_block_from_str(definition.body.as_str())?;
        let block = new_block(deployment.id, block_definition)?;
        engine.blocks.insert(block.id(), block);
    }
    Ok(())
}