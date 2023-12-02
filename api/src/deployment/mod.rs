use actix::Addr;
use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Postgres};

use runtime::engine::engine::EngineActor;
use services::deployment::create::NewDeployment;
use services::deployment::{create, delete, get};

#[get("/deployment/<id>")]
pub async fn get_deployment_handler(
    pool: &State<Pool<Postgres>>,
    id: i32,
) -> Result<String, NotFound<String>> {
    let deployment = get::get_deployment(pool.inner(), id).await;
    match deployment {
        Ok(d) => Ok(serde_json::to_string(&d).unwrap()),
        Err(err) => Err(NotFound(err)),
    }
}

#[post("/deployment", format = "application/json", data = "<dep>")]
pub async fn create_deployment_handler(
    sender: &State<Addr<EngineActor>>,
    pool: &State<Pool<Postgres>>,
    dep: Json<NewDeployment>,
) -> Result<String, Status> {
    match create::create_deployment(sender.inner(), pool.inner(), dep.into_inner()).await {
        Some(deployment) => Ok(serde_json::to_string(&deployment).unwrap()),
        None => Err(Status::InternalServerError),
    }
}

#[delete("/deployment/<id>")]
pub async fn delete_deployment_handler(
    sender: &State<Addr<EngineActor>>,
    pool: &State<Pool<Postgres>>,
    id: i32,
) -> Result<String, Status> {
    let _ = delete::delete_deployment(sender, pool, id).await;
    Ok(id.to_string())
}
