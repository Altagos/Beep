use mongodb::{
    bson::{doc, Bson},
    options::{ClientOptions, FindOneOptions},
    Client, Database,
};
use serenity::{
    client::Context,
    model::id::{GuildId, RoleId},
};

use crate::util::managers::Database as DatabaseManager;

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

pub async fn get_default_role(ctx: &Context, guild_id: &GuildId) -> Option<RoleId> {
    let data = ctx.data.read().await;
    let db = data
        .get::<DatabaseManager>()
        .expect("I expected a database client but got none :(");
    let collection = db.collection("guild_config");
    let filter = doc! {"_id": guild_id.0};
    let mut find_options = FindOneOptions::default();
    find_options.projection = Some(doc! {"default_role": true});
    let result = collection.find_one(filter, find_options).await;

    return match result {
        Ok(document_result) => match document_result {
            Some(document) => {
                if let Some(role_id) = document.get("default_role").and_then(Bson::as_i64) {
                    Some(RoleId(role_id as u64))
                } else {
                    None
                }
            }
            None => None,
        },
        Err(why) => {
            error!("Could not fetch data from guild_config: {}", why);
            None
        }
    };
}
