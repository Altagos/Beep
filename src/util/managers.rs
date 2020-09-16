use crate::util::config::Config;
use dashmap::DashMap;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::{FindOneOptions, FindOptions},
    Collection,
};
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::GuildId,
    prelude::{Mutex, TypeMapKey},
};
use std::{collections::hash_map::RandomState, sync::Arc};

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = Arc<Config>;
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

impl Database {
    pub async fn guild_config_get_id(
        collection: &Collection,
        guild_id: &GuildId,
        key: &str,
    ) -> Option<u64> {
        let mut find_options = FindOneOptions::default();
        find_options.projection = Some(doc! {key: 1});
        let filter = doc! {"_id": guild_id.0, key: {"$exists": true}};
        return match collection.find_one(filter, find_options).await {
            Ok(document) => match document {
                Some(doc) => {
                    if let Some(data) = doc.get(key).and_then(Bson::as_str) {
                        let data = data as &str;
                        match data.parse::<u64>() {
                            Ok(id) => Some(id),
                            Err(why) => {
                                error!("Error: {}", why);
                                None
                            }
                        }
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(why) => {
                error!("Error getting {} from db: {}", key, why);
                None
            }
        };
    }
}
