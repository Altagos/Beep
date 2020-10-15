use mongodb::{options::ClientOptions, Client, Database};

pub fn get_db(client_options: ClientOptions, db_name: &str) -> Database {
    let client = Client::with_options(client_options)
        .expect("Expected client_options for the database connection!");
    client.database(db_name)
}

pub async fn get_db_with_defaults() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    get_db(client_options, "Beep")
}
