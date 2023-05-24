use rocket::Response;
use rocket::response::status::{Created, NotFound};
use services::definition::{create, delete, get};
use rocket::serde::json::Json;
use services::definition::create::NewDefinition;

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

#[delete("/definition/<id>")]
pub fn delete_app_definition_handler(id: i32) -> Result<String, String> {
    let _ = delete::delete_definition(id);
    Ok(id.to_string())
}