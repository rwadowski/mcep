extern crate rocket;

use std::env;
use crossbeam_channel::{Receiver, Sender};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log::{info, LevelFilter};
use rocket::figment::providers::{Env, Format, Toml};
use tokio::signal;
use database;
use api;
use runtime;
use runtime::DataFrame;
use types::deployment::Command;

#[rocket::main]
async fn main() {
    configure_logger();
    info!("Running mcep");

    let (command_tx, command_rx): (Sender<Command>, Receiver<Command>) = crossbeam_channel::unbounded();
    let (kafka_tx, kafka_rx): (Sender<DataFrame>, Receiver<DataFrame>) = crossbeam_channel::unbounded();

    runtime::init();

    let database_connection_pool = database::init_connection_pool().await;

    database::apply_migrations(&database_connection_pool).await.expect("migrations failed");
    tokio::spawn(async move {
        api::start_rocket(rocket_config(), database_connection_pool.clone(), command_tx).launch().await
    });

    let engine_data_input = kafka_rx.clone();
    let engine_data_output = kafka_tx.clone();
    let engine_command_rx = command_rx.clone();
    let app_pool = runtime::pool::create_pool(8).expect("app pool should start");
    app_pool.spawn(move || {
        runtime::engine::run(engine_command_rx, engine_data_input, engine_data_output)
    });
    app_pool.spawn(move || {
        runtime::source::kafka::run_kafka_source(kafka_tx).expect("kafka source should run");
    });
    app_pool.spawn(move || {
        runtime::sink::kafka::run_kafka_sink(kafka_rx).expect("kafka sink should run");
    });
    signal::ctrl_c().await.expect("failed to listen for event");

    println!("Closing mcep");
}

fn rocket_config() -> rocket::figment::Figment {
    rocket::figment::Figment::from(rocket::config::Config::default()).
        merge(Toml::file("Rocket.toml").nested()).
        merge(Env::prefixed("MCEP_").global()).
        select(rocket::figment::Profile::from_env_or("MCEP_PROFILE", "default"))
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
        _ => LevelFilter::Info
    };
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    let _ = log4rs::init_config(config).expect("logger should run");
}