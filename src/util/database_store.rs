use crate::util::{
    database_store::enums::{
        DatabaseCollections, GuildConfigData, GuildConfigKey, StoreResult, TicketKey,
    },
    permission::enums::PermissionOverwriteKey,
};
use mongodb::{
    bson::{doc, Bson},
    options::FindOneOptions,
    Database,
};
use serenity::model::id::{ChannelId, GuildId, RoleId};

#[allow(dead_code)]
pub struct DatabaseStore {
    db: Database,
}

pub mod enums;

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

    #[allow(dead_code, unused_variables)]
    pub async fn get(&self, collection: DatabaseCollections) -> StoreResult {
        return match collection {
            DatabaseCollections::GuildConfig { id, key } => self.gc_handle(id, key).await,
            DatabaseCollections::PermissionOverwrites { id, key } => po_handle(id, key).await,
            DatabaseCollections::Tickets { id, key } => t_handle(id, key).await,
        };
    }

    #[allow(dead_code, unused_variables)]
    async fn gc_handle(&self, id: u64, key: GuildConfigKey) -> StoreResult {
        let collection = self.db.collection("guild_config");
        match key {
            GuildConfigKey::Id => todo!(),
            GuildConfigKey::DefaultRole => {
                let mut find_options = FindOneOptions::default();
                find_options.projection = Some(doc! {"default_role": 1});
                let filter = doc! {"_id": id, "default_role": {"$exists": true}};

                match collection.find_one(filter, find_options).await {
                    Ok(document) => match document {
                        Some(doc) => {
                            if let Some(data) = doc.get("default_role").and_then(Bson::as_i64) {
                                return StoreResult::Guild(GuildConfigData::DefaultRole(RoleId(
                                    data as u64,
                                )));
                            }
                        }
                        None => return StoreResult::None,
                    },
                    Err(why) => {
                        error!("Error getting `default_role` from db: {}", why);
                    }
                };
                return StoreResult::None;
            }
            GuildConfigKey::Prefix => {
                let mut find_options = FindOneOptions::default();
                find_options.projection = Some(doc! {"prefix": 1});
                let filter = doc! {"_id": id, "prefix": {"$exists": true}};

                match collection.find_one(filter, find_options).await {
                    Ok(document) => match document {
                        Some(doc) => {
                            if let Some(data) = doc.get("prefix").and_then(Bson::as_str) {
                                let data = data as &str;
                                return StoreResult::Guild(GuildConfigData::Prefix(
                                    data.to_string(),
                                ));
                            }
                        }
                        None => return StoreResult::None,
                    },
                    Err(why) => {
                        error!("Error getting `ticket_category` from db: {}", why);
                    }
                };
                return StoreResult::None;
            }
            GuildConfigKey::ModerationRole => {
                let mut find_options = FindOneOptions::default();
                find_options.projection = Some(doc! {"moderation_role": 1});
                let filter = doc! {"_id": id, "moderation_role": {"$exists": true}};

                match collection.find_one(filter, find_options).await {
                    Ok(document) => match document {
                        Some(doc) => {
                            if let Some(data) = doc.get("moderation_role").and_then(Bson::as_i64) {
                                return StoreResult::Guild(GuildConfigData::ModerationRole(
                                    RoleId(data as u64),
                                ));
                            }
                        }
                        None => return StoreResult::None,
                    },
                    Err(why) => {
                        error!("Error getting `moderation_role` from db: {}", why);
                    }
                };
                return StoreResult::None;
            }
            GuildConfigKey::TicketCategory => {
                let mut find_options = FindOneOptions::default();
                find_options.projection = Some(doc! {"ticket_category": 1});
                let filter = doc! {"_id": id, "ticket_category": {"$exists": true}};

                match collection.find_one(filter, find_options).await {
                    Ok(document) => match document {
                        Some(doc) => {
                            if let Some(data) = doc.get("ticket_category").and_then(Bson::as_str) {
                                let data = data as &str;
                                match data.parse::<u64>() {
                                    Ok(id) => {
                                        return StoreResult::Guild(GuildConfigData::TicketCategory(
                                            ChannelId(id),
                                        ))
                                    }
                                    Err(why) => {
                                        error!("Error parsing ticket_category id: {}", why);
                                    }
                                }
                            }
                        }
                        None => return StoreResult::None,
                    },
                    Err(why) => {
                        error!("Error getting `ticket_category` from db: {}", why);
                    }
                };
                return StoreResult::None;
            }
        }
    }
}

#[allow(dead_code, unused_variables)]
async fn po_handle(id: u64, key: PermissionOverwriteKey) -> StoreResult {
    todo!();
    StoreResult::None
}

#[allow(dead_code, unused_variables)]
async fn t_handle(id: u64, key: TicketKey) -> StoreResult {
    todo!();
    StoreResult::None
}
