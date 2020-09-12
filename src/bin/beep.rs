use beep::run;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kankyo::load().expect("Failed to load .env file");
    pretty_env_logger::init();

    run().await
}
