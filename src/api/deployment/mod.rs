use crate::services::deployment::DeploymentService;
use crate::types::deployment::{DeploymentId, NewDeployment, UpdateDeployment};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, Scope};

mod mod_test;

pub fn deployment_handler_routes<Service: DeploymentService + Clone + 'static>(
    service: Service,
) -> Scope {
    web::scope("/deployment")
        .app_data(Data::new(service))
        .route("", web::get().to(get_all_deployments_handler::<Service>))
        .route("{id}", web::get().to(get_deployment_handler::<Service>))
        .route("", web::post().to(create_deployment_handler::<Service>))
        .route(
            "{id}",
            web::delete().to(delete_deployment_handler::<Service>),
        )
        .route("", web::patch().to(update_deployment_handler::<Service>))
}

pub async fn get_all_deployments_handler<Service: DeploymentService>(
    service: Data<Service>,
) -> HttpResponse {
    let list = service.get_all().await;
    list.map(|list| HttpResponse::Ok().json(list))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_deployment_handler<Service: DeploymentService>(
    path: Path<DeploymentId>,
    service: Data<Service>,
) -> HttpResponse {
    let id = path.into_inner();
    let deployment = service.get(id).await;
    deployment
        .map(|deployment| HttpResponse::Ok().json(deployment))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn create_deployment_handler<Service: DeploymentService>(
    service: Data<Service>,
    dep: Json<NewDeployment>,
) -> HttpResponse {
    let new_deployment = dep.into_inner();
    let result = service.create(&new_deployment).await;
    result
        .map(|deployment| HttpResponse::Created().json(deployment))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn delete_deployment_handler<Service: DeploymentService>(
    service: Data<Service>,
    path: Path<DeploymentId>,
) -> HttpResponse {
    let id = path.into_inner();
    _ = service.delete(id).await;
    HttpResponse::new(StatusCode::OK)
}

pub async fn update_deployment_handler<Service: DeploymentService>(
    service: Data<Service>,
    dep: Json<UpdateDeployment>,
) -> HttpResponse {
    let update_deployment = dep.into_inner();
    service
        .update(&update_deployment)
        .await
        .map(|deployment| HttpResponse::Ok().json(deployment))
        .unwrap_or_else(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
}
