pub mod app;

use crate::deployment::sink::SinkId;
use crate::deployment::source::SourceId;
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::string::ToString;
use viperus::{Viperus, ViperusValue};

// #[derive(Clone, Serialize, Deserialize)]
// pub struct App {
//     pub database: Database,
//     pub kafka: Kafka,
//     pub logging: Logging,
// }
// 
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Database {
//     pub host: String,
//     pub port: i32,
//     pub user: String,
//     pub password: String,
//     pub name: String,
// }
// 
// impl Database {
//     pub fn from_viperus(v: &Viperus, prefix: &str) -> Result<Database, String> {
//         let host = get_config(v, prefix, "host")?;
//         let port = get_config(v, prefix, "port")?;
//         let user = get_config(v, prefix, "user")?;
//         let password = get_config(v, prefix, "password")?;
//         let name = get_config(v, prefix, "name")?;
//         Ok(Database {
//             host,
//             port,
//             user,
//             password,
//             name,
//         })
//     }
//     pub fn url(&self) -> String {
//         format!(
//             "postgres://{}:{}@{}:{}/{}",
//             self.user, self.password, self.host, self.port, self.name
//         )
//     }
// }
// 
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Kafka {
//     pub hosts: Vec<String>,
//     pub topics: Topics,
//     pub client_id: String,
// }
// 
// impl Kafka {
//     pub fn from_viperus(v: &Viperus, prefix: &str) -> Result<Kafka, String> {
//         let hosts_str: String = get_config(v, prefix, "hosts")?;
//         let hosts = hosts_str
//             .split(",")
//             .into_iter()
//             .map(|s| s.to_string())
//             .collect();
//         let topics = Topics::from_viperus(v, formatted_path(prefix, "topics").as_str())?;
//         let client_id = get_config(v, prefix, "client_id")?;
//         Ok(Kafka {
//             hosts,
//             topics,
//             client_id,
//         })
//     }
//     pub fn sink_id(&self) -> SinkId {
//         SinkId::from(self.client_id.as_str())
//     }
// 
//     pub fn source_id(&self) -> SourceId {
//         SourceId::from(self.client_id.as_str())
//     }
// }
// 
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Topics {
//     pub input: String,
//     pub output: String,
// }
// 
// impl Topics {
//     pub fn from_viperus(v: &Viperus, prefix: &str) -> Result<Topics, String> {
//         let input = get_config(v, prefix, "input")?;
//         let output = get_config(v, prefix, "output")?;
//         Ok(Topics { input, output })
//     }
// }
// 
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Logging {
//     pub debug: bool,
// }
// 
// impl Logging {
//     pub fn from_viperus(v: &Viperus, prefix: &str) -> Result<Logging, String> {
//         let debug = get_config(v, prefix, "debug")?;
//         Ok(Logging { debug })
//     }
// }
// 
// fn formatted_path(prefix: &str, field: &str) -> String {
//     format!("{}.{}", prefix, field)
// }
// 
// fn get_config<'a, T>(v: &Viperus, prefix: &str, field: &str) -> Result<T, String>
// where
//     ViperusValue: From<T>,
//     T: FromStr,
//     T: Clone,
//     &'a ViperusValue: Into<T>,
//     ViperusValue: Into<T>,
// {
//     let path = formatted_path(prefix, field);
//     v.get::<T>(path.as_str())
//         .ok_or(error_message(path.as_str()))
// }
// 
// fn error_message(path: &str) -> String {
//     format!("there is no config at {} path", path)
// }
// 
// pub fn load() -> Result<App, String> {
//     let path: String = env::current_dir()
//         .unwrap()
//         .join("config/dev.toml")
//         .to_str()
//         .unwrap()
//         .to_string();
//     info!("loading config from {}", path.as_str());
//     let mut v = Viperus::new();
//     v.automatic_env(true);
//     v.load_file(path.as_str(), viperus::Format::TOML)
//         .map_err(|err| err.to_string())?;
//     let database = Database::from_viperus(&v, "database")?;
//     let kafka = Kafka::from_viperus(&v, "kafka")?;
//     let logging = Logging::from_viperus(&v, "logging")?;
//     Ok(App {
//         database,
//         kafka,
//         logging,
//     })
// }
// 
// fn from_toml(path: &str) -> Result<App, String> {
//     let contents = from_file(path)?;
//     toml::from_str::<App>(contents.as_str()).map_err(|err| err.to_string())
// }
// 
// fn from_file(path: &str) -> Result<String, String> {
//     let mut file = File::open(path).map_err(|err| err.to_string())?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .map_err(|err| err.to_string())
//         .map(|_| String::new())?;
//     Ok(contents)
// }

// fn tst(path: &str) -> Result<App, String> {
//     let c = Config::builder()
//         .add_source(File::wit)
// }