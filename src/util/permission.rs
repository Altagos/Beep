use crate::util::managers::Database;
use mongodb::{
    bson::{doc, Bson},
    options::FindOneOptions,
};
use serenity::{
    model::{channel::Message, id::RoleId},
    prelude::*,
};

pub async fn _check_moderator_db(
    ctx: &Context,
    msg: &Message,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let guild = msg.guild_id.unwrap();
    let role = {
        let data = ctx.data.read().await;
        let collection = data
            .get::<Database>()
            .unwrap()
            .clone()
            .collection("guild_config");
        let mut find_options = FindOneOptions::default();
        find_options.projection = Some(doc! {"moderation_role": 1});
        match collection
            .find_one(
                doc! {"_id": guild.0, "moderation_role": {"$exists": true}},
                find_options,
            )
            .await
        {
            Ok(document) => match document {
                Some(doc) => {
                    if let Some(role) = doc.get("moderation_role").and_then(Bson::as_i64) {
                        RoleId(role as u64)
                    } else {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            },
            Err(why) => {
                error!("Error getting moderation_role from db: {}", why);
                return Ok(false);
            }
        }
    };

    let author = msg.author.clone();
    Ok(author.has_role(&ctx, guild, role).await?)
}
