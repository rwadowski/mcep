extern crate rocket;

use std::env;

use actix::prelude::*;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use rocket::figment::providers::{Env, Format, Toml};
use tokio::signal;

use api;
use database;
use runtime;
use runtime::engine::EngineActor;
use runtime::sink::kafka::KafkaSinkActor;
use runtime::source::kafka;

#[actix::main]
async fn main() {
    configure_logger();
    info!("Running mcep");

    runtime::init();

    let database_connection_pool = database::init_connection_pool().await;
    let sink = KafkaSinkActor::new().unwrap();
    let engine = EngineActor::new(sink).start();
    let source_target = engine.clone();
    database::apply_migrations(&database_connection_pool)
        .await
        .expect("migrations failed");
    tokio::spawn(async move {
        api::start_rocket(
            rocket_config(),
            database_connection_pool.clone(),
            engine.clone(),
        )
        .launch()
        .await
    });

    let app_pool = runtime::pool::create_pool(8).expect("app pool should start");
    app_pool.spawn(move || {
        let _ = kafka::run_kafka_actor_source(source_target).unwrap();
    });
    app_pool.spawn(move || {
        let system = System::new();
        system.run().unwrap();
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

fn configure_logger() {
    let level = match env::var("DEBUG_ENABLED") {
        Ok(v) => {
            let enabled: bool = v.parse().unwrap_or(false);
            if enabled {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            }
        }
        _ => LevelFilter::Info,
    };
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    let _ = log4rs::init_config(config).expect("logger should run");
}
