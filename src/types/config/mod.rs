use crate::types::deployment::sink::SinkId;
use crate::types::deployment::source::SourceId;
use config::{Config, Environment, File};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Serialize, Deserialize)]
pub struct App {
    pub database: Database,
    pub kafka: Kafka,
    pub logging: Logging,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl Database {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Kafka {
    pub hosts: String,
    pub topics: Topics,
    pub client_id: String,
}

impl Kafka {
    pub fn sink_id(&self) -> SinkId {
        let formatted = format!("{}-sink", self.client_id.as_str());
        SinkId::from(formatted.as_str())
    }

    pub fn source_id(&self) -> SourceId {
        let formatted = format!("{}-source", self.client_id.as_str());
        SourceId::from(formatted.as_str())
    }

    pub fn host_list(&self) -> Vec<String> {
        self.hosts.split(",").map(|s| s.to_string()).collect()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Topics {
    pub input: String,
    pub output: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Logging {
    pub debug: bool,
}

pub fn load() -> Result<App, String> {
    let path: String = env::current_dir()
        .unwrap()
        .join("config/dev.toml")
        .to_str()
        .unwrap()
        .to_string();
    let environment = Environment::default().separator("_");
    let c = Config::builder()
        .add_source(File::with_name(path.as_str()))
        .add_source(environment)
        .build()
        .map_err(|err| err.to_string())?;

    c.try_deserialize().map_err(|err| err.to_string())
}
