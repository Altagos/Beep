use std::{collections::hash_map::RandomState, sync::Arc};

use dashmap::DashMap;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::GuildId,
    prelude::{Mutex, TypeMapKey},
};

use crate::util::config::Config;

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = Config;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Database;

impl TypeMapKey for Database {
    type Value = mongodb::Database;
}

pub struct Prefixes;

impl TypeMapKey for Prefixes {
    type Value = Arc<DashMap<GuildId, String>>;
}

impl Prefixes {
    pub async fn load(db: mongodb::Database) -> DashMap<GuildId, String, RandomState> {
        info!("Loading prefixes...");
        let prefixes = DashMap::default();

        let collection = db.collection("guild_config");
        let mut find_options = FindOptions::default();
        find_options.projection = Some(doc! {"_id": true, "prefix": true});
        let mut cursor = collection
            .find(doc! {"prefix": {"$exists": true}}, find_options)
            .await
            .unwrap();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let guild_id = document.get("_id").and_then(Bson::as_i64).unwrap();
                    let guild_id = GuildId(guild_id as u64);
                    let prefix = document.get("prefix").and_then(Bson::as_str).unwrap();
                    prefixes.insert(guild_id, prefix.to_string());
                }
                Err(why) => {
                    error!("Could not load prefixes from db: {}", why);
                }
            }
        }

        info!("Loaded prefixes");
        prefixes
    }
}
