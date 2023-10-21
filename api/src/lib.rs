mod definition;
mod deployment;

#[macro_use] extern crate rocket;

use crossbeam_channel::Sender;
use rocket::{Build, Rocket};
use rocket::figment::Figment;
use sqlx::{Pool, Postgres};
use types::deployment::Command;

pub fn start_rocket(config: Figment, pool: Pool<Postgres>, sender: Sender<Command>) -> Rocket<Build> {
    rocket::custom(config)
        .manage(pool)
        .manage(sender)
        .mount("/api", routes![
            definition::get_app_definition_handler,
            definition::create_app_definition_handler,
            definition::delete_app_definition_handler,
            definition::update_app_definition_handler,
            deployment::get_deployment_handler,
            deployment::create_deployment_handler,
            deployment::delete_deployment_handler,
        ])
}