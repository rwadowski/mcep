mod api;
mod database;
mod runtime;
mod services;
mod types;
mod utils;

extern crate rocket;

use actix::prelude::*;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use rocket::figment::providers::{Env, Format, Toml};
use tokio::signal;

use runtime::engine::EngineActor;
use runtime::sink::kafka::KafkaSinkActor;
use runtime::source::SourceActor;
use types::config;
use types::config::Logging;

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
    let engine = EngineActor::new(sink).start();
    let engine_actor = engine.clone();
    info!("starting source");
    SourceActor::new(&config.kafka, engine_actor)
        .unwrap()
        .start();

    tokio::spawn(async move {
        api::start_rocket(
            rocket_config(),
            database_connection_pool.clone(),
            engine.clone(),
        )
        .launch()
        .await
    });
    signal::ctrl_c().await.expect("failed to listen for event");
    println!("Closing mcep");
}

fn rocket_config() -> rocket::figment::Figment {
    rocket::figment::Figment::from(rocket::config::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("MCEP_").global())
        .select(rocket::figment::Profile::from_env_or(
            "MCEP_PROFILE",
            "default",
        ))
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
