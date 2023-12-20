use rocket::log::private::{error, info};
use rocket::serde::{Deserialize, Serialize};

use serde_json::Value;
use sqlx::types::JsonValue;
use sqlx::{Error, Pool, Postgres, Type};

use crate::types::definition::Definition;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDefinition {
    pub name: String,
    pub version: String,
    pub body: Value,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Type)]
#[sqlx(transparent)]
struct Body(JsonValue);

pub async fn create_definition(pool: &Pool<Postgres>, def: NewDefinition) -> Option<Definition> {
    let result: Result<Definition, Error> = sqlx::query_as::<_, Definition>("INSERT INTO definitions (name, version, body, description, help) VALUES ($1, $2, $3, $4, $5) RETURNING *;")
        .bind(def.name)
        .bind(def.version)
        .bind(def.body)
        .bind(def.description)
        .bind(def.help)
        .fetch_one(pool)
        .await;

    match result {
        Ok(created_def) => {
            info!("definition {} created", created_def.id.to_string());
            Some(created_def)
        }
        Err(err) => {
            error!("{}", err.to_string());
            None
        }
    }
}
