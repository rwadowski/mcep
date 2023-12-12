use crate::deployment::sink::SinkId;
use crate::deployment::source::SourceId;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use viperus::{Viperus, ViperusValue};

//TODO - use simply toml create instead toml_env ???
//Yaml file ?
#[derive(Clone, Serialize, Deserialize)]
pub struct App {
    pub database: Database,
    pub kafka: Kafka,
    pub logging: Logging,
}

impl From<App> for ViperusValue {
    fn from(value: App) -> Self {
        let str = toml::to_string(&value).ok();
        str.map(|s| ViperusValue::Str(s))
            .unwrap_or(ViperusValue::Empty)
    }
}

impl FromStr for App {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match toml::from_str::<App>(s) {
            Ok(app) => Ok(app),
            Err(_) => Err(()),
        }
    }
}

impl Into<App> for ViperusValue {
    fn into(self) -> App {
        value_to_app(&self)
    }
}

impl Into<App> for &ViperusValue {
    fn into(self) -> App {
        value_to_app(self)
    }
}

fn value_to_app(value: &ViperusValue) -> App {
    match value {
        ViperusValue::Str(s) => toml::from_str::<App>(s.as_str()).unwrap(),
        _ => panic!("what I should do?"),
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u64,
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
    println!("{}", path.clone());
    let mut v = Viperus::new();
    v.automatic_env(true);
    let res = v.load_file(path.as_str(), viperus::Format::TOML);
    match res {
        Ok(()) => (),
        Err(err) => println!("{}", err.to_string()),
    }
    let c = v.get::<App>("database");
    c.ok_or("failed to read the config".to_string())
}
