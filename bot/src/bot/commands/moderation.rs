use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::UserId, prelude::Mentionable},
    prelude::Context,
};

#[command]
#[only_in(guilds)]
#[required_permissions(KICK_MEMBERS)]
#[min_args(1)]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let user = args.single::<UserId>().unwrap();

    if user.0 == msg.author.id.0 {
        if let Err(why) = msg.reply(&ctx.http, "you cannot kick yourself.").await {
            error!("Error sending message: {:?}", why);
        }
        return Ok(());
    }

    if let Err(why) = guild.kick(&ctx.http, &user).await {
        error!("Could not kick user from guild: {}", why);
    } else {
        if let Err(why) = msg
            .reply(
                &ctx.http,
                format!("user {} was kicked from this guild", &user.mention()),
            )
            .await
        {
            error!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}
