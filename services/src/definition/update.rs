use sqlx::{Error, Pool, Postgres};
use rocket::serde::Deserialize;
use types::definition::Definition;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateDefinition {
    pub id: i32,
    pub version: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub body_type: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

pub async fn update_definition<'a>(pool: &Pool<Postgres>, def: UpdateDefinition) -> Option<Definition> {
    let id = def.id;
    let (query_str, values) = query_data(def);
    let mut query = sqlx::query_as::<_, Definition>(query_str.as_str());
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(id);
    let result: Result<Definition, Error> = query.fetch_one(pool).await;
    match result {
        Ok(definition) => Some(definition),
        Err(_) => None
    }
}

fn query_data(def: UpdateDefinition) -> (String, Vec<String>) {
    let mut query_str = "UPDATE app_definitions SET ".to_string();
    let mut set_clauses: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut index = 1;
    if let Some(title) = def.title {
        let q = format!("{} = ${}", "title", index);
        set_clauses.push(q);
        values.push(title);
        index = index + 1;
    }
    if let Some(body) = def.body {
        let q = format!("{} = ${}", "body", index);
        set_clauses.push(q);
        values.push(body);
        index = index + 1;
    }
    if let Some(description) = def.description {
        let q = format!("{} = ${}", "description", index);
        set_clauses.push(q);
        values.push(description);
        index = index + 1;
    }
    if let Some(help) = def.help {
        let q = format!("{} = ${}", "help", index);
        set_clauses.push(q);
        values.push(help);
        index = index + 1;
    }
    if let Some(version) = def.version {
        let q = format!("{} = ${}", "version", index);
        set_clauses.push(q);
        values.push(version);
        index = index + 1;
    }
    if let Some(body_type) = def.body_type {
        let q = format!("{} = ${}", "body_type", index);
        set_clauses.push(q);
        values.push(body_type);
        index = index + 1;
    }
    query_str.push_str(set_clauses.join(", ").as_str());
    query_str.push_str(format!(" WHERE id = ${} RETURNING *;", index).as_str());
    (query_str, values)
}