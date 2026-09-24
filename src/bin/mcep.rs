use actix_files::Files;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use log::info;
use mcep::api::definition::definition_handler_routes;
use mcep::api::deployment::deployment_handler_routes;
use mcep::database::psql::definition::PsqlDefinitionStore;
use mcep::database::psql::deployment::PsqlDeploymentStore;
use mcep::runtime::engine::Engine;
use mcep::runtime::sink::kafka::spawn_sink;
use mcep::runtime::source::spawn_source;
use mcep::services::definition::{DefinitionService, Service};
use mcep::services::deployment::{DeploymentService, Service as DeploymentServiceImpl};
use mcep::types::config;
use mcep::{database, runtime, utils};
use tokio::signal;

#[actix_web::main]
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

    let nats = async_nats::connect(&config.nats.host)
        .await
        .expect("nats connection failed");

    info!("starting sink");
    spawn_sink(&config.kafka, nats.clone())
        .await
        .expect("sink must start");

    let engine = Engine::new(nats.clone());

    let definition_store = PsqlDefinitionStore::new(database_connection_pool.clone());
    let definition_service = Service::new(definition_store.clone());
    let deployment_store = PsqlDeploymentStore::new(database_connection_pool.clone());
    let deployment_service =
        DeploymentServiceImpl::new(deployment_store, definition_store, engine.clone());

    info!("starting engine");
    let definitions = definition_service
        .get_all()
        .await
        .expect("definitions should be loaded");
    let deployments = deployment_service
        .get_all()
        .await
        .expect("deployments should be loaded");

    {
        let definitions_by_id: std::collections::HashMap<_, _> =
            definitions.into_iter().map(|d| (d.id, d)).collect();
        for dep in deployments {
            engine
                .deploy(&dep, &definitions_by_id)
                .await
                .expect("deployment must succeed");
        }
    }

    info!("starting source");
    spawn_source(&config.kafka, nats.clone()).expect("source must start");

    let server = HttpServer::new(move || {
        let v1 = web::scope("/v1")
            .service(definition_handler_routes(definition_service.clone()))
            .service(deployment_handler_routes(deployment_service.clone()));
        let api = web::scope("/api").service(v1);
        App::new()
            .wrap(ActixLogger::default())
            .app_data(Data::new(database_connection_pool.clone()))
            .app_data(Data::new(engine.clone()))
            .service(api)
            .service(
                Files::new("/", "./frontend/dist")
                    .index_file("index.html")
                    .default_handler(web::get().to(spa_fallback)),
            )
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();
    let server_handle = server.handle();
    actix_web::rt::spawn(server);
    signal::ctrl_c().await.expect("failed to listen for event");
    server_handle.stop(false).await;
    info!("closing mcep");
}

async fn spa_fallback() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./frontend/dist/index.html")?)
}
