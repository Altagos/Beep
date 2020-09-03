use rand::Rng;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use crate::util::send;

#[command]
pub async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount_dice = args.single::<String>()?;
    let amount_dice: Vec<&str> = amount_dice.split(|c| c == 'd' || c == 'D').collect();

    let times = match amount_dice[0].parse::<u32>() {
        Ok(times) => times,
        Err(_) => {
            send(
                ctx,
                &msg.channel_id,
                "I'm sorry but you did not give me a number for the amount of the rolls!",
            )
            .await;
            return Ok(());
        }
    };

    let dice = match amount_dice[1].parse::<u32>() {
        Ok(dice) => dice,
        Err(_) => {
            send(
                ctx,
                &msg.channel_id,
                "I'm sorry but you did not give me a valid dice!",
            )
            .await;
            return Ok(());
        }
    };

    let roll = Roll::new(times, dice);

    send(
        ctx,
        &msg.channel_id,
        &*format!(
            "You rolled a d{} {} time{} and got a `{}`",
            roll.dice,
            if roll.times >= 2 { "s" } else { "" },
            roll.times,
            roll.roll()
        ),
    )
    .await;
    Ok(())
}

struct Roll {
    times: u32,
    dice: u32,
}

impl Roll {
    pub fn new(times: u32, dice: u32) -> Self {
        Self { times, dice }
    }

    pub fn roll(&self) -> u32 {
        let mut roll: u32 = 0;
        let mut rng = rand::thread_rng();

        for _ in 0..self.times {
            roll += rng.gen_range(1, &self.dice + 1);
        }

        roll
    }
}
