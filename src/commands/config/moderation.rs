use crate::util::{managers::Database, send};
use mongodb::{bson::doc, options::UpdateOptions};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::Context,
};

#[command]
#[only_in(guild)]
#[required_permissions(ADMINISTRATOR)]
#[owner_privilege(true)]
#[num_args(1)]
async fn moderation_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let _mod_role = args.single::<RoleId>()?;
    let guild_id = msg.guild(ctx).await.unwrap();

    let collection = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .unwrap()
            .clone()
            .collection("guild_config")
    };

    let role = stringify!(_mod_role.0);

    let filter = doc! {"_id": guild_id.id.0};
    let doc = doc! {"$set": {"moderation_role": role}};
    let mut options = UpdateOptions::default();
    options.upsert = Some(true);

    if let Err(why) = collection.update_one(filter, doc, options).await {
        send(
            ctx,
            &msg.channel_id,
            "Could not update the moderation role for this server",
        )
        .await;
        error!(
            "Unable to update moderation_role for {}: {}",
            guild_id.id.0, why
        );
        return Ok(());
    }

    send(
        ctx,
        &msg.channel_id,
        "Updated the moderation role for this server",
    )
    .await;
    Ok(())
}
