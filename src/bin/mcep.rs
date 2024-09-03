use actix::prelude::*;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::web::Data;
use actix_web::{rt, web, App, HttpServer};
use log::info;
use log4rs::config::Root;
use mcep::api::{definition, deployment};
use mcep::runtime::engine::EngineActor;
use mcep::runtime::sink::kafka::KafkaSinkActor;
use mcep::runtime::source::SourceActor;
use mcep::types::config;
use mcep::types::definition::Definition;
use mcep::types::deployment::Deployment;
use mcep::{database, runtime, utils};
use sqlx::{Pool, Postgres};
use tokio::signal;

#[actix::main]
async fn main() {
    let config = config::load().expect("config should be loaded");
    utils::configure_logger(&config.logging);
    info!("running mcep");
    if config.logging.debug {
        info!("logs in debug mode");
    }

    runtime::init();

    let database_connection_pool = database::init_connection_pool(&config.database).await;
    database::apply_migrations(&database_connection_pool)
        .await
        .expect("migrations failed");

    info!("starting sink");
    let sink = KafkaSinkActor::new(&config.kafka).unwrap();
    info!("starting engine");
    let (definitions, deployments) = load(&database_connection_pool)
        .await
        .expect("database state must be loaded");
    let engine = EngineActor::new(sink, definitions, deployments)
        .expect("engine must start")
        .start();
    info!("starting source");
    SourceActor::new(&config.kafka, engine.clone())
        .unwrap()
        .start();

    let server = HttpServer::new(move || {
        let definition_services = web::scope("/definition")
            .service(definition::create_app_definition_handler)
            .service(definition::get_app_definition_handler)
            .service(definition::delete_app_definition_handler)
            .service(definition::update_app_definition_handler)
            .service(definition::get_all_definitions_handler);
        let deployment_services = web::scope("/deployment")
            .service(deployment::create_deployment_handler)
            .service(deployment::get_deployment_handler)
            .service(deployment::delete_deployment_handler);
        let v1 = web::scope("/v1")
            .service(definition_services)
            .service(deployment_services);
        let api = web::scope("/api").service(v1);
        App::new()
            .wrap(ActixLogger::default())
            .app_data(Data::new(database_connection_pool.clone()))
            .app_data(Data::new(engine.clone()))
            .service(api)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run();
    let server_handle = server.handle();
    rt::spawn(server);
    signal::ctrl_c().await.expect("failed to listen for event");
    server_handle.stop(false).await;
    info!("closing mcep");
}

async fn load(pool: &Pool<Postgres>) -> Result<(Vec<Definition>, Vec<Deployment>), String> {
    let definitions = mcep::services::definition::get::get_all_definitions(pool)
        .await
        .map_err(utils::to_string)?;
    let deployments = mcep::services::deployment::get::get_all_deployments(pool)
        .await
        .map_err(utils::to_string)?;
    Ok((definitions, deployments))
}
