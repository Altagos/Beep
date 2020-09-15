use mongodb::bson::doc;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use crate::util::{managers::Database, send};
use mongodb::options::FindOneOptions;
use serenity::model::{
    channel::ChannelCategory,
    id::{ChannelId, RoleId},
    prelude::*,
};

#[command]
#[min_args(1)]
#[only_in(guild)]
async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let ticket_collection = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .unwrap()
            .clone()
            .collection("tickets")
    };
    let gc_collection = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .unwrap()
            .clone()
            .collection("guild_config")
    };

    let title = args.single_quoted::<String>()?;
    let channel_title = title.clone();
    let description = args.rest().to_string();
    let guild_id = msg.guild_id.unwrap();
    let author_id = msg.author.id;

    let moderation_role =
        match Database::guild_config_get_id(&gc_collection, &guild_id, "moderation_role").await {
            Some(role_id) => RoleId(role_id),
            _ => {
                send(
                    ctx,
                    &msg.channel_id,
                    "Could not create a new ticket for you",
                )
                .await;
                error!("Error: Mod");
                return Ok(());
            }
        };

    let ticket_category =
        match Database::guild_config_get_id(&gc_collection, &guild_id, "ticket_category").await {
            Some(category_id) => ChannelId(category_id),
            _ => {
                send(
                    ctx,
                    &msg.channel_id,
                    "Could not create a new ticket for you",
                )
                .await;
                error!("Error: Ticket");
                return Ok(());
            }
        };

    let author_perms = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::SEND_TTS_MESSAGES,
        kind: PermissionOverwriteType::Member(author_id),
    };

    let mod_perms = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::SEND_TTS_MESSAGES,
        kind: PermissionOverwriteType::Role(moderation_role),
    };

    let everyone_perms = PermissionOverwrite {
        allow: Permissions::STREAM,
        deny: Permissions::READ_MESSAGES,
        kind: PermissionOverwriteType::Role(
            msg.guild(&ctx)
                .await
                .unwrap()
                .role_by_name("@everyone")
                .unwrap()
                .id,
        ),
    };
    let guild = msg.guild(&ctx).await.unwrap();
    let cat = guild.channels.get(&ticket_category);
    info!("Cat: {:?}", cat);

    let channel = guild_id
        .create_channel(&ctx, |c| {
            c.name(channel_title)
                .category::<ChannelId>(ticket_category)
                .permissions::<Vec<PermissionOverwrite>>(vec![
                    author_perms,
                    everyone_perms,
                    mod_perms,
                ])
        })
        .await;
    match channel {
        Ok(c) => {
            let insert = doc! {
                "guild_id": guild_id.0,
                "channel_id": c.id.0,
                "author_id": author_id.0,
                "title": title,
                "description": description
            };

            if let Err(why) = ticket_collection.insert_one(insert, None).await {
                send(
                    ctx,
                    &msg.channel_id,
                    "Could not create a new ticket for you",
                )
                .await;
                error!("Unable to create a ticket for {}: {}", guild_id.0, why);
                return Ok(());
            }
        }
        Err(why) => {
            send(
                ctx,
                &msg.channel_id,
                "Could not create a new ticket for you",
            )
            .await;
            error!("Unable to create a ticket for {}: {}", guild_id.0, why);
            return Ok(());
        }
    }

    Ok(())
}
