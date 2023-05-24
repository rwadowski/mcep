use tokio::signal::ctrl_c;
use tokio::signal;
use api;

#[tokio::main]
async fn main() {
    println!("Running mcep");
    std::thread::spawn(move || api::start_rocket());

    signal::ctrl_c().await.expect("failed to listen for event");

    println!("Closing mcep");
}

