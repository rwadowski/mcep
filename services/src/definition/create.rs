use rocket::response::status::Created;
use rocket::serde::json::Json;
use definition::Definition;
use rocket::serde;
use serde::Deserialize;
use sqlx::{Error, Executor, Pool, Postgres};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDefinition {
    pub title: String,
    pub version: String,
    pub body: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

pub async fn create_definition(pool: &Pool<Postgres>, def: NewDefinition) -> Option<Definition> {
    let result: Result<Definition, Error> = sqlx::query_as::<_, Definition>("INSERT INTO app_definitions (title, version, body, description, help) VALUES ($1, $2, $3, $4, $5) RETURNING *;")
        .bind(def.title)
        .bind(def.version)
        .bind(def.body)
        .bind(def.description)
        .bind(def.help)
        .fetch_one(pool)
        .await;

    match result {
        Ok(created_def) => Some(created_def),
        _ => None
    }
}