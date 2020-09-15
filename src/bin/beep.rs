use beep::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kankyo::load().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    run().await
}
