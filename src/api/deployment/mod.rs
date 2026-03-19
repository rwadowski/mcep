use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, HttpResponse};
use sqlx::{Pool, Postgres};

use crate::runtime::engine::Engine;
use crate::services::deployment::{create, delete, get};
use crate::services::deployment::create::NewDeployment;

mod mod_test;

#[get("{id}")]
pub async fn get_deployment_handler(path: Path<i32>, pool: Data<Pool<Postgres>>) -> HttpResponse {
    let id = path.into_inner();
    let deployment = get::get_deployment(&pool, id).await;
    match deployment {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND),
    }
}

#[post("")]
pub async fn create_deployment_handler(
    engine: Data<Arc<Engine>>,
    pool: Data<Pool<Postgres>>,
    dep: Json<NewDeployment>,
) -> HttpResponse {
    match create::create_deployment(&engine, &pool, dep.into_inner()).await {
        Some(deployment) => HttpResponse::Ok().json(deployment),
        None => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[delete("{id}")]
pub async fn delete_deployment_handler(
    path: Path<i32>,
    engine: Data<Arc<Engine>>,
    pool: Data<Pool<Postgres>>,
) -> HttpResponse {
    let id = path.into_inner();
    let _ = delete::delete_deployment(&engine, &pool, id).await;
    HttpResponse::new(StatusCode::OK)
}
