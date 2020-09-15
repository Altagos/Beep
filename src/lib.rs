extern crate tracing_subscriber;
#[macro_use]
extern crate log;
extern crate serenity;

use std::{collections::HashSet, fs, sync::Arc};

use serenity::{
    client::Context,
    framework::{
        standard::{
            help_commands,
            macros::{command, group, help, hook},
            Args, CommandGroup, CommandResult, DispatchError, HelpOptions,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, prelude::*},
    Client,
};
use tokio::time::{delay_for, Duration};

use crate::util::{
    config::Config as BotConfig,
    db::get_db_with_defaults,
    groups::*,
    handler::Handler,
    managers::{BotConfig as BotConfigData, Database, Prefixes, ShardManagerContainer},
};

mod commands;
mod util;

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
#[lacking_ownership = "Hide"]
#[wrong_channel = "Strike"]
#[embed_success_colour(DARK_BLUE)]
#[indention_prefix = ">"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn after(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    if let Err(why) = error {
        let error_string = "Looks like the bot encountered an error! \n";

        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.color(0xff69b4);
                    e.title("Aw Snap!");
                    e.description(error_string);
                    e.field("Command Name", cmd_name, false);
                    e.field("Error", format!("```{:?} \n```", why), false);
                    e
                })
            })
            .await;
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::LackingPermissions(Permissions::ADMINISTRATOR) => {
            let _ = msg.channel_id.say(ctx, "Not for you!").await;
        }
        DispatchError::LackingPermissions(Permissions::MANAGE_MESSAGES) => {
            let _ = msg.channel_id.say(ctx,
                                       "You can't execute this command because you aren't a moderator! (Manage Messages permission)").await;
        }
        DispatchError::LackingPermissions(Permissions::BAN_MEMBERS) => {
            let _ = msg.channel_id.say(ctx,
                                       "You can't execute this command because you aren't a moderator! (Ban Members permission)").await;
        }
        DispatchError::LackingPermissions(Permissions::KICK_MEMBERS) => {
            let _ = msg.channel_id.say(ctx,
                                       "You can't execute this command because you aren't a moderator! (Kick Members permission)").await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    ctx,
                    format!("Args required: {}. Args given: {}", min, given),
                )
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(ctx, "This is a bot dev only command!")
                .await;
        }
        DispatchError::IgnoredBot => {}
        _ => println!("Unhandled dispatch error: {:?}", error),
    }
}

#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let data = ctx.data.read().await;
    let prefixes = data.get::<Prefixes>().unwrap();

    if msg.guild_id.is_some() {
        return match prefixes.get(&msg.guild_id.unwrap()) {
            Some(prefix) => Some(prefix.value().to_string()),
            _ => None,
        };
    }
    None
}

#[group]
#[commands(ping)]
struct General;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("Config.toml").expect("I expect a Config.toml file!");
    let bot_config: BotConfig = toml::from_str(&config_file).unwrap();

    let token = &bot_config.bot.token;
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefix("!")
                .on_mention(Some(bot_id))
                .owners(owners)
                .dynamic_prefix(dynamic_prefix)
                .delimiters(vec![", ", ","])
        })
        .help(&MY_HELP)
        .after(after)
        .on_dispatch_error(dispatch_error)
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP)
        .group(&CONFIG_GROUP)
        .group(&TICKET_GROUP)
        .group(&DND_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(150)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                info!(
                    "Shard {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });

    let insert_bot_config = bot_config.clone();
    let db = get_db_with_defaults().await;
    {
        let mut data = client.data.write().await;

        data.insert::<BotConfigData>(Arc::new(insert_bot_config));
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<Database>(db.clone());
        data.insert::<Prefixes>(Arc::new(Prefixes::load(db).await));
    }

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<Database>().unwrap();

    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
