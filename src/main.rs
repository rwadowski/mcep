#[macro_use] extern crate rocket;

use tokio::signal::ctrl_c;
use tokio::signal;
use api;

#[rocket::main]
async fn main() {
    println!("Running mcep");
    tokio::spawn(async move {
        api::start_rocket().launch().await
    });

    signal::ctrl_c().await.expect("failed to listen for event");

    println!("Closing mcep");
}

