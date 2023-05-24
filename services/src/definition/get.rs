use definition::Definition;
use database::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn get_app_definition(id: i32) -> Result<Definition, NotFound<String>> {
    use definition::schema::app_definitions;

    match app_definitions::table.find(id).first::<Definition>(&mut establish_connection()) {
        Ok(app) => Ok(app),
        Err(err) => match err {
            diesel::result::Error::NotFound => Result::Err(NotFound(err.to_string())),
            _ => panic!("Database error - {}", err),
        }
    }
}