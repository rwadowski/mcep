mod definition;

#[macro_use] extern crate rocket;

use rocket::{Build, Error, Ignite, Rocket, tokio};

pub fn start_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes![
            definition::app::get_app_definition_handler,
            definition::app::create_app_definition_handler,
            definition::app::delete_app_definition_handler,
        ])
}