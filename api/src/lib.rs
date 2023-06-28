mod definition;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::figment::Figment;
use sqlx::{Pool, Postgres};

pub fn start_rocket(config: Figment, pool: Pool<Postgres>) -> Rocket<Build> {
    rocket::custom(config)
        .manage(pool)
        .mount("/api", routes![
            definition::app::get_app_definition_handler,
            definition::app::create_app_definition_handler,
            definition::app::delete_app_definition_handler,
            definition::app::update_app_definition_handler,
        ])
}