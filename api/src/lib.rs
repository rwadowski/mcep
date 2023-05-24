mod definition;

#[macro_use] extern crate rocket;

use rocket::{Error, Ignite, Rocket, tokio};

pub async fn start() -> Result<Rocket<Ignite>, Error> {
    rocket::build()
        .mount("/api", routes![
            definition::app::get_app_definition_handler,
            definition::app::create_app_definition_handler,
        ])
        .launch()
        .await
}

pub fn start_rocket() -> std::io::Result<i32> {
    let rt = tokio::runtime::Runtime::new()?;

    let builder = rocket::build()
        .mount("/api", routes![
            definition::app::get_app_definition_handler,
            definition::app::create_app_definition_handler,
        ]);
    rt.block_on(async move {
        let _ = builder.launch().await;
    });
    Ok(0)
}