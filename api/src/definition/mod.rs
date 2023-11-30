use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use services::definition::create::NewDefinition;
use services::definition::update::UpdateDefinition;
use services::definition::{create, delete, get, update};
use sqlx::{Pool, Postgres};

#[get("/definition/<id>")]
pub async fn get_app_definition_handler(
    pool: &State<Pool<Postgres>>,
    id: i32,
) -> Result<String, NotFound<String>> {
    let definition = get::get_definition(pool.inner(), id).await;
    match definition {
        Ok(d) => Ok(serde_json::to_string(&d).unwrap()),
        Err(err) => Err(NotFound(err)),
    }
}

#[post("/definition", format = "application/json", data = "<def>")]
pub async fn create_app_definition_handler(
    pool: &State<Pool<Postgres>>,
    def: Json<NewDefinition>,
) -> Result<String, Status> {
    match create::create_definition(pool.inner(), def.into_inner()).await {
        Some(definition) => Ok(serde_json::to_string(&definition).unwrap()),
        None => Err(Status::InternalServerError),
    }
}

#[delete("/definition/<id>")]
pub async fn delete_app_definition_handler(
    pool: &State<Pool<Postgres>>,
    id: i32,
) -> Result<String, String> {
    let _ = delete::delete_definition(pool.inner(), id).await;
    Ok(id.to_string())
}

#[patch("/definition", format = "application/json", data = "<def>")]
pub async fn update_app_definition_handler(
    pool: &State<Pool<Postgres>>,
    def: Json<UpdateDefinition>,
) -> Result<String, Status> {
    match update::update_definition(pool.inner(), def.into_inner()).await {
        Some(definition) => Ok(serde_json::to_string(&definition).unwrap()),
        None => Ok("".to_string()),
    }
}
