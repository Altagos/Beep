use crate::util::{
    database_store::enums::{DatabaseCollections, GuildConfigData, GuildConfigKey, StoreResult},
    embed_store::{EmbedStore, TicketEmbed},
    managers::{Database, DatabaseStore},
};
use mongodb::bson::doc;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, RoleId},
        prelude::*,
    },
    prelude::Context,
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
    let database_store = {
        let data = ctx.data.read().await;
        data.get::<DatabaseStore>().unwrap().clone()
    };

    let guild = msg.guild(&ctx).await.unwrap();

    let title = args.single_quoted::<String>()?;
    let channel_title = title.clone();
    let description = args.rest().to_string();
    let guild_id = msg.guild_id.unwrap();
    let author_id = msg.author.id;

    // let moderation_role =
    //     match Database::guild_config_get_id(&gc_collection, &guild_id, "moderation_role").await {
    //         Some(role_id) => RoleId(role_id),
    //         _ => {
    //             EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
    //             return Ok(());
    //         }
    //     };
    let moderation_role = match database_store
        .get(DatabaseCollections::GuildConfig {
            id: guild_id.0,
            key: GuildConfigKey::ModerationRole,
        })
        .await
    {
        StoreResult::Guild(data) => {
            if let GuildConfigData::ModerationRole(role) = data {
                role
            } else {
                EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
                return Ok(());
            }
        }
        _ => {
            EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
            return Ok(());
        }
    };

    let ticket_category =
        match Database::guild_config_get_id(&gc_collection, &guild_id, "ticket_category").await {
            Some(category_id) => category_id,
            _ => {
                EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
                return Ok(());
            }
        };

    let category = match guild.channels.get(&ChannelId(ticket_category)) {
        Some(cat) => cat,
        None => {
            EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
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

    let channel = guild_id
        .create_channel(&ctx, |c| {
            c.name(channel_title)
                // .category(ticket_category)
                .permissions::<Vec<PermissionOverwrite>>(vec![
                    author_perms,
                    everyone_perms,
                    mod_perms,
                ])
                .kind(ChannelType::Text)
                .topic(&description)
                .category(category)
        })
        .await;

    match channel {
        Ok(c) => {
            let insert = doc! {
                "guild_id": &guild_id.0,
                "channel_id": &c.id.0,
                "author_id": &author_id.0,
                "title": &title,
                "description": &description
            };

            if let Err(why) = ticket_collection.insert_one(insert, None).await {
                EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
                error!("Unable to create a ticket for {}: {}", guild_id.0, why);
                return Ok(());
            } else {
                let author: User = author_id.to_user(&ctx).await?;
                EmbedStore::ticket(
                    TicketEmbed::Description {
                        author,
                        title,
                        description,
                    },
                    ctx,
                    &c.id,
                )
                .await;
                EmbedStore::ticket(TicketEmbed::Success(c), ctx, &msg.channel_id).await;
            }
        }
        Err(why) => {
            EmbedStore::ticket(TicketEmbed::Failure, ctx, &msg.channel_id).await;
            error!("Unable to create a ticket for {}: {}", guild_id.0, why);
            return Ok(());
        }
    }

    Ok(())
}
