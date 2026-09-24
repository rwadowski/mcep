use crate::services::definition::DefinitionService;
use crate::types::definition::{DefinitionId, NewDefinition, UpdateDefinition};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, Scope};

mod mod_test;

pub fn definition_handler_routes<Service: DefinitionService + Clone + 'static>(
    service: Service,
) -> Scope {
    web::scope("/definition")
        .app_data(Data::new(service))
        .route("", web::get().to(get_all_definitions_handler::<Service>))
        .route(
            "{id}",
            web::get().to(get_app_definitions_handler::<Service>),
        )
        .route("", web::post().to(create_app_definition_handler::<Service>))
        .route(
            "{id}",
            web::delete().to(delete_app_definition_handler::<Service>),
        )
        .route(
            "",
            web::patch().to(update_app_definition_handler::<Service>),
        )
}

pub async fn get_all_definitions_handler<Service: DefinitionService>(
    service: Data<Service>,
) -> HttpResponse {
    let empty = Vec::<DefinitionId>::new();
    let list = service.get_list(&empty).await;
    list.map(|list| HttpResponse::Ok().json(list))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_app_definitions_handler<Service: DefinitionService>(
    path: Path<i32>,
    service: Data<Service>,
) -> HttpResponse {
    let id = path.into_inner();
    let definition = service.get(id).await;
    definition
        .map(|definition| HttpResponse::Ok().json(definition))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn create_app_definition_handler<Service: DefinitionService>(
    service: Data<Service>,
    def: Json<NewDefinition>,
) -> HttpResponse {
    let definition = def.into_inner();
    let result = service.create(&definition).await;
    result
        .map(|definition| HttpResponse::Created().json(definition))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}
pub async fn delete_app_definition_handler<Service: DefinitionService>(
    service: Data<Service>,
    path: Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    _ = service.delete(id).await;
    HttpResponse::new(StatusCode::OK)
}

pub async fn update_app_definition_handler<Service: DefinitionService>(
    service: Data<Service>,
    def: Json<UpdateDefinition>,
) -> HttpResponse {
    let update_definition = def.into_inner();
    service
        .update(&update_definition)
        .await
        .map(|definition| HttpResponse::Ok().json(definition))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}
