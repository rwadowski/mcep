use rocket::Response;
use rocket::response::status::{Created, NotFound};
use definition::NewDefinition;
use services::definition::{create, get};
use rocket::serde::json::Json;

#[get("/definition/<id>")]
pub fn get_app_definition_handler(id: i32) -> Result<String, NotFound<String>> {

    let definition = get::get_app_definition(id);
    match definition {
        Ok(d) => Ok(serde_json::to_string(&d).unwrap()),
        Err(err) => Err(err)
    }
}

#[post("/definition", format="application/json", data="<def>")]
pub fn create_app_definition_handler(def: Json<NewDefinition>) -> Created<String> {
    create::create_definition(def)
}