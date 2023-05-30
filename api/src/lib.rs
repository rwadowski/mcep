mod definition;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use sqlx::{Pool, Postgres};

pub fn start_rocket(pool: Pool<Postgres>) -> Rocket<Build> {
    rocket::build()
        .manage(pool)
        .mount("/api", routes![
            definition::app::get_app_definition_handler,
            definition::app::create_app_definition_handler,
            definition::app::delete_app_definition_handler,
            definition::app::update_app_definition_handler,
        ])
}