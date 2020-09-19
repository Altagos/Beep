use serenity::{
    client::Context,
    http::Http,
    model::id::{ChannelId, UserId},
};

pub mod config;
pub mod database_store;
pub mod db;
pub mod embed_store;
pub mod groups;
pub mod handler;
pub mod managers;
pub mod permission;

pub async fn send(ctx: &Context, channel: &ChannelId, content: &str) {
    if let Err(why) = channel.say(ctx, content).await {
        error!("Could not send message: {}", why);
    }
}

pub async fn get_bot_id(http: &Http) -> UserId {
    return match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
}
