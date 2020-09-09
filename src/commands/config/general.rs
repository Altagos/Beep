use mongodb::{bson::doc, options::UpdateOptions};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::{Context, Mentionable},
};

use crate::util::{
    managers::{BotConfig, Database, Prefixes},
    send,
};

///Use this command to setup the bot for this guild
#[command]
#[only_in(guild)]
#[required_permissions(ADMINISTRATOR)]
#[owner_privilege(true)]
async fn setup(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    send(ctx, &msg.channel_id, "Not implemented yet").await;
    Ok(())
}

/// Configure a role that is automatically given to new users.
///
/// Usage:
/// ```discord
/// !c default_role <MENTION_THE_ROLE>
/// !c auto_role <MENTION_THE_ROLE>
/// ```
#[command]
#[only_in(guild)]
#[required_permissions(MANAGE_GUILD)]
#[owner_privilege(true)]
#[num_args(1)]
#[aliases("auto_role")]
async fn default_role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let default_role = args.parse::<RoleId>()?;
    let guild_id = msg.guild(ctx).await.unwrap();

    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>().unwrap().clone()
    };
    let collection = db.collection("guild_config");

    let filter = doc! {"_id": guild_id.id.0};
    let doc = doc! {"$set": {"default_role": default_role.0}};
    let mut options = UpdateOptions::default();
    options.upsert = Some(true);

    if let Err(why) = collection.update_one(filter, doc, options).await {
        send(
            ctx,
            &msg.channel_id,
            "Could not update the default role for this server",
        )
        .await;
        error!(
            "Unable to update default_role for {}: {}",
            guild_id.id.0, why
        );
        return Ok(());
    }

    send(
        ctx,
        &msg.channel_id,
        &*format!(
            "The default role for this server is now {}",
            default_role.mention()
        ),
    )
    .await;
    Ok(())
}

/// Set a custom prefix for your server.
/// You can also reset the prefix for this bot by using `reset` instead of a custom prefix
///
/// Usage:
/// ```discord
/// !config prefix <custom_prefix>
/// !config prefix `reset`
/// ```
#[command]
#[only_in(guild)]
#[required_permissions(MANAGE_GUILD)]
#[owner_privilege(true)]
#[num_args(1)]
#[aliases("p")]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix = args.single::<String>()?;
    let guild_id = msg.guild_id.unwrap();

    let (prefixes, bot_config, db) = {
        let data = ctx.data.read().await;
        let prefixes = data.get::<Prefixes>().unwrap().clone();
        let bot_config = data.get::<BotConfig>().unwrap().clone();
        let db = data.get::<Database>().unwrap().clone();
        (prefixes, bot_config, db)
    };
    let collection = db.collection("guild_config");

    if prefix != "`reset`" && prefix != bot_config.bot.default_prefix {
        let filter = doc! {"_id": guild_id.0};
        let doc = doc! {"$set": {"prefix": &prefix}};
        let mut options = UpdateOptions::default();
        options.upsert = Some(true);

        if let Err(why) = collection.update_one(filter, doc, options).await {
            send(
                ctx,
                &msg.channel_id,
                "Could not update the prefix for this server",
            )
            .await;
            error!("Unable to update prefix for {}: {}", guild_id, why);
            return Ok(());
        }

        prefixes.insert(guild_id, String::from(&prefix));
        send(
            ctx,
            &msg.channel_id,
            &*format!("My prefix for this server is now: {}", prefix),
        )
        .await;
    } else {
        let filter = doc! {"_id": guild_id.0};
        let doc = doc! {"$unset": {"prefix": ""}};

        if let Err(why) = collection.update_one(filter, doc, None).await {
            send(
                ctx,
                &msg.channel_id,
                "Could not reset the prefix for this server",
            )
            .await;
            error!("Unable to reset prefix for {}: {}", guild_id, why);
            return Ok(());
        }

        prefixes.remove(&guild_id);
        send(
            ctx,
            &msg.channel_id,
            &*format!("My prefix for this server is now my default prefix"),
        )
        .await;
    }
    Ok(())
}
