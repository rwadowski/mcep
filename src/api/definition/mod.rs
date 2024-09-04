use crate::services::definition::create::NewDefinition;
use crate::services::definition::update::UpdateDefinition;
use crate::services::definition::{create, delete, get, update};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, patch, post, HttpResponse};
use sqlx::{Pool, Postgres};

#[get("")]
pub async fn get_all_definitions_handler(pool: Data<Pool<Postgres>>) -> HttpResponse {
    let list = get::get_all_definitions(&pool).await;
    match list {
        Ok(definitions) => HttpResponse::Ok().json(definitions),
        _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[get("{id}")]
pub async fn get_app_definition_handler(
    path: Path<i32>,
    pool: Data<Pool<Postgres>>,
) -> HttpResponse {
    let id = path.into_inner();
    let definition = get::get_definition(&pool, id).await;
    match definition {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND),
    }
}

#[post("")]
pub async fn create_app_definition_handler(
    pool: Data<Pool<Postgres>>,
    def: Json<NewDefinition>,
) -> HttpResponse {
    match create::create_definition(&pool, def.into_inner()).await {
        Ok(definition) => HttpResponse::Ok().json(definition),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[delete("{id}")]
pub async fn delete_app_definition_handler(
    path: Path<i32>,
    pool: Data<Pool<Postgres>>,
) -> HttpResponse {
    let id = path.into_inner();
    let _ = delete::delete_definition(&pool, id).await;
    HttpResponse::new(StatusCode::OK)
}

#[patch("")]
pub async fn update_app_definition_handler(
    pool: Data<Pool<Postgres>>,
    def: Json<UpdateDefinition>,
) -> HttpResponse {
    match update::update_definition(&pool, def.into_inner()).await {
        Ok(definition) => HttpResponse::Ok().json(definition),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
