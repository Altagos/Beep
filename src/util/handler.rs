use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        prelude::{GuildId, Member},
    },
};

use crate::util::{db::get_default_role, get_bot_id};

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
        if new_member.user.id == get_bot_id(ctx.http()).await {
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

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
