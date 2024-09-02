extern crate core;

use chrono::Utc;
use clap::{Parser, ValueEnum};
use log::{error, info};
use mcep::runtime::engine::Data;
use mcep::runtime::{Message, Name};
use mcep::tools::deployments::new_deployment;
use mcep::types::config::Logging;
use mcep::types::definition::{Definition, DefinitionId};
use mcep::types::deployment::DeploymentId;
use mcep::{tools, utils};
use std::str::FromStr;

#[derive(ValueEnum, Parser, Clone, Debug)]
enum Action {
    Send,
    CreateDefinition,
    DeleteDefinition,
    Deploy,
    Undeploy,
}

#[derive(ValueEnum, Parser, Clone, Debug)]
enum AppDefinition {
    Sum,
    Store,
    Rename,
}
/// Management tool for mcep
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Action to do
    action: Action,

    /// Github token
    #[arg(long)]
    token: Option<String>,

    /// Mcep host
    #[arg(long)]
    host: Option<String>,

    /// Mcep port
    #[arg(long)]
    port: Option<i32>,

    #[arg(long)]
    /// App definition to be deployed
    definition: Option<AppDefinition>,

    #[arg(long)]
    /// Deployment configuration
    deployment: Option<String>,

    #[arg(long)]
    /// Deployment name
    name: Option<String>,

    #[arg(long)]
    /// Sources in deployment
    source: Option<Vec<String>>,

    #[arg(long)]
    /// Sinks in deployment
    sink: Option<Vec<String>>,

    #[arg(long)]
    /// Blocks in deployment
    block: Option<Vec<String>>,

    #[arg(long)]
    /// Connections in deployment
    connection: Option<Vec<String>>,

    #[arg(long)]
    /// Deployment, definition id
    id: Option<i32>,

    #[arg(long)]
    /// Dataframe name
    df_name: Option<Vec<String>>,

    #[arg(long)]
    /// Dataframe value type
    df_value_type: Option<Vec<String>>,

    #[arg(long)]
    /// Dataframe value
    df_value: Option<Vec<String>>,

    #[arg(long)]
    /// Kafka host
    kafka_host: Option<Vec<String>>,

    #[arg(long)]
    /// Kafka topic
    kafka_topic: Option<String>,
}

fn main() {
    info!("tools, parsing params");
    let cli = Cli::parse();

    utils::configure_logger(&Logging { debug: true });

    let result = match cli.action.clone() {
        Action::CreateDefinition => {
            info!("creating definition");
            let definition_id = create_definition(&cli);
            definition_id.map(|id| format!("created definition with id {}", id))
        }
        Action::DeleteDefinition => {
            info!("deleting definition");
            delete_definition(&cli)
        }
        Action::Deploy => {
            info!("deploying");
            let deployment_id = create_deployment(&cli);
            deployment_id.map(|id| format!("deployed deployment with id {}", id))
        }
        Action::Undeploy => {
            info!("undeploying");
            delete_deployment(&cli)
        }
        Action::Send => {
            info!("sending to kafka");
            send(&cli)
        }
        _ => Err("not implemented yet".to_string()),
    };
    match result {
        Ok(txt) => info!("{}", txt),
        Err(msg) => error!("{}", msg),
    }
}

fn create_definition(cli: &Cli) -> Result<DefinitionId, String> {
    let app_definition = cli.definition.clone().ok_or("no definition provided")?;
    let token = cli.token.clone().ok_or("no token provided")?;
    let definition = get_definition(app_definition, token)?;
    let host = cli.host.clone().unwrap_or(String::from("localhost"));
    let port = cli.port.clone().unwrap_or(8080);
    tools::apply::create_definition(host, port, definition)
}

fn get_definition(d: AppDefinition, token: String) -> Result<Definition, String> {
    match d {
        AppDefinition::Sum => tools::definitions::new_sum_block_definition(token),
        AppDefinition::Store => tools::definitions::new_store_block_definition(token),
        _ => Err("there is no definition implemented in tools".to_string()),
    }
}

fn delete_definition(cli: &Cli) -> Result<String, String> {
    let definition_id = cli.id.clone().ok_or("no definition provided")?;
    let host = cli.host.clone().unwrap_or(String::from("localhost"));
    let port = cli.port.clone().unwrap_or(8080);
    let result = tools::apply::delete_definition(host, port, definition_id);
    result.map(|_| format!("{} definition successfully deleted", definition_id))
}

fn create_deployment(cli: &Cli) -> Result<DeploymentId, String> {
    let name = cli
        .name
        .clone()
        .ok_or("there is no deployment app".to_string())?;
    let sources = cli.source.clone().unwrap_or(Vec::new());
    let sinks = cli.sink.clone().unwrap_or(Vec::new());
    let blocks = cli.block.clone().unwrap_or(Vec::new());
    let connections = cli.connection.clone().unwrap_or(Vec::new());
    let deployment = new_deployment(name, sources, sinks, blocks, connections)?;
    let host = cli.host.clone().unwrap_or(String::from("localhost"));
    let port = cli.port.clone().unwrap_or(8080);
    tools::apply::create_deployment(host, port, deployment)
}

fn delete_deployment(cli: &Cli) -> Result<String, String> {
    let deployment_id = cli.id.clone().ok_or("no deployment app".to_string())?;
    let host = cli.host.clone().unwrap_or(String::from("localhost"));
    let port = cli.port.clone().unwrap_or(8080);
    let result = tools::apply::delete_deployment(host, port, deployment_id);
    result.map(|_| format!("{} deployment successfully deleted", deployment_id))
}

fn send(cli: &Cli) -> Result<String, String> {
    let names = cli.df_name.clone().ok_or("no df names".to_string())?;
    let value_types = cli
        .df_value_type
        .clone()
        .ok_or("no df value types".to_string())?;
    let values = cli.df_value.clone().ok_or("no df value".to_string())?;
    let hosts = cli.kafka_host.clone().ok_or("no kafka host".to_string())?;
    let topic = cli
        .kafka_topic
        .clone()
        .ok_or("no kafka topic".to_string())?;
    let size = names.len();
    if size != values.len() || size != value_types.len() {
        return Err("incorrect input, values or value types length mismatch".to_string());
    }
    let messages = to_kafka_messages(names, values, value_types)?;
    let result = tools::apply::send(hosts, topic, messages);
    result.map(|size| format!("{} send successfully", size))
}

fn to_kafka_messages(
    names: Vec<String>,
    values: Vec<String>,
    types: Vec<String>,
) -> Result<Vec<Message>, String> {
    let size = names.len();
    if size != values.len() || size != types.len() {
        return Err("incorrect input, values or value types length mismatch".to_string());
    }
    let mut messages: Vec<Message> = Vec::new();
    for i in 0..size {
        let name = names[i].clone();
        let value = values[i].clone();
        let value_type = types[i].clone();
        messages.push(to_kafka_message(name, value, value_type)?);
    }
    Ok(messages)
}

fn to_kafka_message(name: String, value: String, r#type: String) -> Result<Message, String> {
    match r#type.to_lowercase().as_str() {
        "text" => Ok(Message {
            ts: Utc::now(),
            name: Name { value: name },
            value: Data::Text(value),
        }),
        "bool" => {
            let v = bool::from_str(value.as_str()).map_err(utils::to_string)?;
            Ok(Message {
                ts: Utc::now(),
                name: Name { value: name },
                value: Data::Boolean(v),
            })
        }
        "unsignedint" => {
            let v = u64::from_str(value.as_str()).map_err(utils::to_string)?;
            Ok(Message {
                ts: Utc::now(),
                name: Name { value: name },
                value: Data::UnsignedInt(v),
            })
        }
        "signedint" => {
            let v = i64::from_str(value.as_str()).map_err(utils::to_string)?;
            Ok(Message {
                ts: Utc::now(),
                name: Name { value: name },
                value: Data::SignedInt(v),
            })
        }
        "float" => {
            let v = f64::from_str(value.as_str()).map_err(utils::to_string)?;
            Ok(Message {
                ts: Utc::now(),
                name: Name { value: name },
                value: Data::Float(v),
            })
        }
        _ => Err(format!("unknown type {}", r#type)),
    }
}
