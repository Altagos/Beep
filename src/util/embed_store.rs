use serenity::{
    client::Context,
    model::{
        channel::{GuildChannel, Message},
        misc::Mentionable,
    },
    utils::Color,
};

pub struct EmbedStore;

impl EmbedStore {
    // pub async fn basic(ctx: &Context, msg: &Message, title: &str, description: &str) {
    //     if let Err(why) = msg
    //         .channel_id
    //         .send_message(&ctx, |m| {
    //             m.embed(|e| e.title(title).description(description))
    //         })
    //         .await
    //     {
    //         error!(
    //             "Error sending message (channel_id: {}): {}",
    //             msg.channel_id, why
    //         );
    //     }
    // }

    // Ticket System
    pub async fn ticket_failure(ctx: &Context, msg: &Message) {
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Error")
                        .description("Error creating a ticket for you!")
                        .color(Color::RED)
                })
            })
            .await
        {
            error!(
                "Error sending message (channel_id: {}): {}",
                msg.channel_id, why
            );
        }
    }

    pub async fn ticket_success(ctx: &Context, msg: &Message, ticket_channel: &GuildChannel) {
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("New Ticket")
                        .description(format!("Please go to {}", ticket_channel.mention()))
                        .color(Color::from_rgb(73, 248, 22))
                })
            })
            .await
        {
            error!(
                "Error sending message (channel_id: {}): {}",
                msg.channel_id, why
            );
        }
    }
}
