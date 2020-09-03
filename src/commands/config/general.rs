use mongodb::{bson::doc, options::UpdateOptions};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::{Context, Mentionable},
};

use crate::util::{managers::Database, send};

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

    let data = ctx.data.read().await;
    let db = data
        .get::<Database>()
        .expect("I expected a database client but got none :(");
    let collection = db.collection("guild_config");

    let filter = doc! {"_id": guild_id.id.0};
    let doc = doc! {"default_role": default_role.0};
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
