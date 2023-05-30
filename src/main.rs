extern crate rocket;

use tokio::signal;
use database;
use api;

#[rocket::main]
async fn main() {
    println!("Running mcep");

    let database_connection_pool = database::init_connection_pool().await;

    database::apply_migrations(&database_connection_pool).await.expect("migrations failed");

    tokio::spawn(async move {
        api::start_rocket(database_connection_pool).launch().await
    });

    signal::ctrl_c().await.expect("failed to listen for event");

    println!("Closing mcep");
}

