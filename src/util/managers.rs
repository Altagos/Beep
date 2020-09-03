use std::sync::Arc;

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Database;

impl TypeMapKey for Database {
    type Value = mongodb::Database;
}
