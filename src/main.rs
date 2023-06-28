extern crate rocket;

use rocket::figment::providers::{Env, Format, Toml};
use tokio::signal;
use database;
use api;

#[rocket::main]
async fn main() {
    println!("Running mcep");

    let database_connection_pool = database::init_connection_pool().await;

    database::apply_migrations(&database_connection_pool).await.expect("migrations failed");
    tokio::spawn(async move {
        api::start_rocket(rocket_config(), database_connection_pool).launch().await
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
