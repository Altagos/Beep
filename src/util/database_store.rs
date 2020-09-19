use mongodb::{
    bson::{doc, Bson},
    options::FindOneOptions,
    Database,
};
use serenity::model::id::{GuildId, RoleId};

pub struct DatabaseStore {
    db: Database,
}

pub enum DatabaseCollections {
    GuildConfig { id: u64, key: String },
    Prefixes { id: u64, key: String },
    PermissionOverwrites { id: u64, key: String },
    Tickets { id: u64, key: String },
}

impl DatabaseStore {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_default_role(&self, guild_id: GuildId) -> Option<RoleId> {
        let collection = self.db.collection("guild_config");
        let filter = doc! {"_id": guild_id.0};
        let mut find_options = FindOneOptions::default();
        find_options.projection = Some(doc! {"default_role": 1});
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

    pub async fn get<T>(&self, collection: DatabaseCollections) -> Option<T> {
        match collection {
            DatabaseCollections::GuildConfig { id, key } => {}
            DatabaseCollections::Prefixes { id, key } => {}
            DatabaseCollections::PermissionOverwrites { id, key } => {}
            DatabaseCollections::Tickets { id, key } => {}
        }
        Some(0 as T)
    }
}
