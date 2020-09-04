use mongodb::bson::doc;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::ResumedEvent,
        gateway::Ready,
        prelude::{GuildId, Member},
        user::User,
    },
};

use crate::util::{db::get_default_role, get_bot_id, managers::Database};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            info!(
                "{} is connected on shard {}/{}!",
                ready.user.name,
                shard[0] + 1,
                shard[1],
            );
        }
        // info!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, mut new_member: Member) {
        if new_member.user.bot {
            return;
        }
        let default_role = get_default_role(&ctx, &guild_id).await;
        match default_role {
            Some(default_role) => {
                if let Err(why) = new_member.add_role(ctx, default_role).await {
                    error!("Could not add role to member: {}", why);
                }
            }
            _ => {}
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        let data = ctx.data.write().await;
        let db = data
            .get::<Database>()
            .expect("I expected a database client but got none :(");
        if user.id == get_bot_id(&ctx.http).await {
            let collection = db.collection("guild_config");
            let filter = doc! {"_id": guild_id.0};
            if let Err(why) = collection.find_one_and_delete(filter, None).await {
                error!("Could not delete guild from db: {}", why);
            } else {
                info!("Delete guild from db");
            }
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
