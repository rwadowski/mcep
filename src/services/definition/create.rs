use rocket::log::private::{error, info};
use rocket::serde::{Deserialize, Serialize};

use crate::types::definition::block::{Block, BlockType};
use serde_json::Value;
use sqlx::{Error, Pool, Postgres};

use crate::types::definition::Definition;
use crate::utils;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDefinition {
    pub name: String,
    pub version: String,
    pub body: Box<dyn Block>, // <- to jest code block - to poprawic
    // pub body: Value, // <- to jest code block - to poprawic
    pub description: Option<String>,
    pub help: Option<String>,
}

pub async fn create_definition(
    pool: &Pool<Postgres>,
    def: NewDefinition,
) -> Result<Definition, String> {
    let body = def.body.as_json()?;
    let result: Result<Definition, Error> = sqlx::query_as::<_, Definition>("INSERT INTO definitions (name, version, body, description, help) VALUES ($1, $2, $3, $4, $5) RETURNING *;")
        .bind(def.name)
        .bind(def.version)
        .bind(body)
        .bind(def.description)
        .bind(def.help)
        .fetch_one(pool)
        .await;

    match result {
        Ok(created_def) => {
            info!("definition {} created", created_def.id.to_string());
            Ok(created_def)
        }
        Err(err) => {
            error!("{}", err.to_string());
            Err(err.to_string())
        }
    }
}
