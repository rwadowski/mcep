use rocket::serde::Deserialize;
use sqlx::{Error, Pool, Postgres};
use types::definition::Definition;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewDefinition {
    pub name: String,
    pub version: String,
    pub body: String,
    pub description: Option<String>,
    pub help: Option<String>,
}

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
        Ok(created_def) => Some(created_def),
        Err(err) => {
            println!("{}", err.to_string());
            None
        }
    }
}