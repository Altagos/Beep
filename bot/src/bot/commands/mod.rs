use crate::bot::util::send;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

pub mod config;
pub mod dnd;
pub mod moderation;
pub mod ticket;

#[command]
async fn basic_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    send(ctx, &msg.channel_id, "Hello world").await;
    Ok(())
}
