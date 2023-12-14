use crate::deployment::sink::SinkId;
use crate::deployment::source::SourceId;
use config::{Config, Environment, File};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Serialize, Deserialize)]
pub struct App {
    pub database: Database,
    pub kafka: Kafka,
    pub logging: Logging,
    pub debug: bool,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Kafka {
    pub hosts: Vec<String>,
    pub topics: Topics,
    pub client_id: String,
}

impl Kafka {
    pub fn sink_id(&self) -> SinkId {
        SinkId::from(self.client_id.as_str())
    }

    pub fn source_id(&self) -> SourceId {
        SourceId::from(self.client_id.as_str())
    }
}

#[derive(Clone, Serialize, Deserialize)]
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
    let c = Config::builder()
        .add_source(File::with_name(path.as_str()))
        .add_source(Environment::with_prefix("MCEP"))
        .build()
        .map_err(|err| err.to_string())?;

    //TODO - use this, fix resolution of envsŻ
    let x = c.get_bool("debug").unwrap();
    let y = env::var("MCEP_DEBUG").unwrap();
    c.try_deserialize().map_err(|err| err.to_string())
}
