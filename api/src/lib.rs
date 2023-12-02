#[macro_use]
extern crate rocket;

use actix::Addr;
use rocket::figment::Figment;
use rocket::{Build, Rocket};
use runtime::engine::engine::EngineActor;
use sqlx::{Pool, Postgres};

mod definition;
mod deployment;

pub fn start_rocket(
    config: Figment,
    pool: Pool<Postgres>,
    sender: Addr<EngineActor>,
) -> Rocket<Build> {
    rocket::custom(config).manage(pool).manage(sender).mount(
        "/api",
        routes![
            definition::get_app_definition_handler,
            definition::create_app_definition_handler,
            definition::delete_app_definition_handler,
            definition::update_app_definition_handler,
            deployment::get_deployment_handler,
            deployment::create_deployment_handler,
            deployment::delete_deployment_handler,
        ],
    )
}
