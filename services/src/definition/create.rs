use rocket::response::status::Created;
use rocket::serde::json::Json;
use definition::{Definition, NewDefinition};
use database::establish_connection;
use diesel::prelude::*;

pub fn create_definition(def: Json<NewDefinition>) -> Created<String> {
    use definition::schema::app_definitions;

    let d = def.into_inner();

    match diesel::insert_into(app_definitions::table).values(&d).get_result::<Definition>(&mut establish_connection()) {
        Ok(d) =>
            Created::new(d.id.to_string()),
        Err(err) =>  panic!("Database error - {}", err)
    }
}