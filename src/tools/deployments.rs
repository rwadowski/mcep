use crate::types::definition::DataType;
use crate::types::deployment::connection::junction::BlockJunction;
use crate::types::deployment::connection::BlockConnection;
use crate::types::deployment::sink::{Sink, SinkId};
use crate::types::deployment::source::{Source, SourceId};
use crate::types::deployment::{BlockId, DeployedBlock, Deployment};
use crate::utils;
use std::collections::HashSet;
use std::str::FromStr;

pub fn new_deployment(
    name: String,
    sources: Vec<String>,
    sinks: Vec<String>,
    blocks: Vec<String>,
    connections: Vec<String>,
) -> Result<Deployment, String> {
    let sources_list = parse_sources(sources)?;
    let sinks_list = parse_sinks(sinks)?;
    let connections_list = parse_connections(&sources_list, &sinks_list, connections)?;
    let deployment = Deployment {
        id: 0,
        name,
        version: "tools-created".to_string(),
        connections: connections_list,
        sources: sources_list.clone(),
        sinks: sinks_list.clone(),
        blocks: parse_blocks(blocks)?,
    };
    Ok(deployment)
}

fn parse_connections(
    sources: &Vec<Source>,
    sinks: &Vec<Sink>,
    list: Vec<String>,
) -> Result<Vec<BlockConnection>, String> {
    let source_id_set = sources
        .iter()
        .map(|s| s.id.clone())
        .collect::<HashSet<SourceId>>();
    let sink_id_set = sinks
        .iter()
        .map(|s| s.id.clone())
        .collect::<HashSet<SinkId>>();
    let mut result = Vec::<BlockConnection>::new();
    for s in list {
        let connection = parse_connection(&source_id_set, &sink_id_set, s)?;
        result.push(connection);
    }
    Ok(result)
}

fn parse_connection(
    sources: &HashSet<SourceId>,
    sinks: &HashSet<SinkId>,
    string: String,
) -> Result<BlockConnection, String> {
    let from_to = string.split("->").collect::<Vec<&str>>();
    if from_to.len() != 2 {
        return Err("incorrect connection format".to_string());
    }
    let from = from_to[0].split(":").collect::<Vec<&str>>();
    if from.len() != 2 {
        return Err("incorrect connection format".to_string());
    }
    let to = from_to[1].split(":").collect::<Vec<&str>>();
    if to.len() != 2 {
        return Err("incorrect connection format".to_string());
    }
    let block_connection = BlockConnection {
        from: block_junction(sources, sinks, from)?,
        to: block_junction(sources, sinks, to)?,
    };
    Ok(block_connection)
}

fn block_junction(
    sources: &HashSet<SourceId>,
    sinks: &HashSet<SinkId>,
    elements: Vec<&str>,
) -> Result<BlockJunction, String> {
    let block = elements[0];
    let dt = elements[1];
    let source = sources.get(&SourceId::from(block)).map(|s| s.clone());
    let sink = sinks.get(&SinkId::from(block)).map(|s| s.clone());
    let block = match (source.clone(), sink.clone()) {
        (None, None) => {
            let id = BlockId::try_from(block).map_err(utils::to_string)?;
            Some(id)
        }
        _ => None,
    };
    let junction = BlockJunction {
        block,
        sink,
        source,
        data_type: data_type(dt)?,
    };
    Ok(junction)
}

fn parse_blocks(list: Vec<String>) -> Result<Vec<DeployedBlock>, String> {
    let mut blocks = Vec::<DeployedBlock>::new();
    for s in list {
        let block = parse_block(s)?;
        blocks.push(block);
    }
    Ok(blocks)
}

fn parse_block(string: String) -> Result<DeployedBlock, String> {
    DeployedBlock::from_str(string.as_str()).map_err(utils::to_string)
}
fn parse_sources(list: Vec<String>) -> Result<Vec<Source>, String> {
    let mut result = Vec::<Source>::new();
    for s in list {
        let source = parse_source(s)?;
        result.push(source);
    }
    Ok(result)
}

fn parse_source(string: String) -> Result<Source, String> {
    let elements = string.split(":").collect::<Vec<&str>>();
    if elements.len() != 2 {
        return Err("incorrect source format".to_string());
    }
    let source = Source {
        id: SourceId::from(elements[0]),
        data_type: data_type(elements[1])?,
    };
    Ok(source)
}
fn parse_sinks(list: Vec<String>) -> Result<Vec<Sink>, String> {
    let mut result = Vec::<Sink>::new();
    for s in list {
        let sink = parse_sink(&s)?;
        result.push(sink);
    }
    Ok(result)
}

fn parse_sink(string: &String) -> Result<Sink, String> {
    let elements = string.split(":").collect::<Vec<&str>>();
    if elements.len() != 2 {
        return Err("incorrect sink format".to_string());
    }
    let sink = Sink {
        id: SinkId::from(elements[0]),
        data_type: data_type(elements[1])?,
    };
    Ok(sink)
}

fn data_type(string: &str) -> Result<DataType, String> {
    match string.to_lowercase().as_str() {
        "boolean" => Ok(DataType::Boolean),
        "text" => Ok(DataType::Text),
        "signed_int" => Ok(DataType::SignedInt),
        "unsigned_int" => Ok(DataType::UnsignedInt),
        "float" => Ok(DataType::Float),
        _ => Err(format!("unknown data type: {}", string)),
    }
}
