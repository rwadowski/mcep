extern crate rocket;

use crossbeam_channel::{Receiver, Sender};
use rocket::figment::providers::{Env, Format, Toml};
use tokio::signal;
use database;
use api;
use runtime;
use runtime::DataFrame;

#[rocket::main]
async fn main() {
    println!("Running mcep");

    let database_connection_pool = database::init_connection_pool().await;

    database::apply_migrations(&database_connection_pool).await.expect("migrations failed");
    tokio::spawn(async move {
        api::start_rocket(rocket_config(), database_connection_pool).launch().await
    });

    let (tx, rx): (Sender<DataFrame>, Receiver<DataFrame>) = crossbeam_channel::unbounded();

    let app_pool = runtime::pool::create_pool(8).expect("app pool should start");
    app_pool.spawn(move || {
        runtime::source::kafka::run_kafka_source(tx).expect("kafka source should run");
    });
    app_pool.spawn(move || {
        runtime::sink::kafka::run_kafka_sink(rx).expect("kafka sink should run");
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
