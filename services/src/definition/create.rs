use rocket::response::status::Created;
use rocket::serde::json::Json;
use definition::Definition;
use database::establish_connection;
use diesel::prelude::*;
use rocket::serde;
use serde::Deserialize;
use definition::schema::app_definitions;

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = app_definitions)]
pub struct NewDefinition {
    pub title: String,
    pub version: String,
    pub body: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

pub fn create_definition(def: Json<NewDefinition>) -> Created<String> {
    use definition::schema::app_definitions;

    let d = def.into_inner();

    match diesel::insert_into(app_definitions::table).values(&d).get_result::<Definition>(&mut establish_connection()) {
        Ok(d) =>
            Created::new(d.id.to_string()),
        Err(err) =>  panic!("Database error - {}", err)
    }
}