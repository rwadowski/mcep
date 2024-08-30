mod api;
mod database;
mod runtime;
mod services;
mod types;
mod utils;

use actix::prelude::*;
use actix_web::web::Data;
use actix_web::{rt, web, App, HttpServer};
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use sqlx::{Pool, Postgres};
use tokio::signal;

use crate::types::definition::Definition;
use crate::types::deployment::Deployment;
use runtime::engine::EngineActor;
use runtime::sink::kafka::KafkaSinkActor;
use runtime::source::SourceActor;
use types::config;
use types::config::Logging;
use crate::api::{definition, deployment};

#[actix::main]
async fn main() {
    let config = config::load().expect("config should be loaded");
    configure_logger(&config.logging);
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
        let definition_services = web::scope("definition")
            .service(definition::get_app_definition_handler)
            .service(definition::create_app_definition_handler)
            .service(definition::delete_app_definition_handler)
            .service(definition::update_app_definition_handler);
        let deployment_services = web::scope("deployment")
            .service(deployment::create_deployment_handler)
            .service(deployment::get_deployment_handler)
            .service(deployment::delete_deployment_handler);
        App::new()
            .app_data(Data::new(database_connection_pool.clone()))
            .app_data(Data::new(engine.clone()))
            .service(definition_services)
            .service(deployment_services)
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

fn configure_logger(logging: &Logging) {
    let level = if logging.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("kafka::consumer", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    let _ = log4rs::init_config(config).expect("logger should run");
}

async fn load(pool: &Pool<Postgres>) -> Result<(Vec<Definition>, Vec<Deployment>), String> {
    let definitions = services::definition::get::get_all_definitions(pool)
        .await
        .map_err(utils::to_string)?;
    let deployments = services::deployment::get::get_all_deployments(pool)
        .await
        .map_err(utils::to_string)?;
    Ok((definitions, deployments))
}
