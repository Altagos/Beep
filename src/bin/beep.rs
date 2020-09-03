use beep::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kankyo::load().expect("Failed to load .env file");
    env_logger::init();

    run().await
}
