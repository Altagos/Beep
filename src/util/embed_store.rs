use serenity::{
    client::Context,
    model::{channel::GuildChannel, id::ChannelId, misc::Mentionable, user::User},
    utils::Color,
};

pub enum TicketEmbed {
    Success(GuildChannel),
    Description {
        author: User,
        title: String,
        description: String,
    },
    Failure,
}

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

    pub async fn ticket(state: TicketEmbed, ctx: &Context, channel_id: &ChannelId) {
        if let Err(why) = channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| match state {
                    TicketEmbed::Success(c) => e
                        .title("New Ticket")
                        .description(format!("Please go to {}", c.mention()))
                        .color(Color::from_rgb(73, 248, 22)),
                    TicketEmbed::Failure => e
                        .title("Error")
                        .description("Error creating a ticket for you!")
                        .color(Color::RED),
                    TicketEmbed::Description {
                        author,
                        title,
                        description: desc,
                    } => e
                        .author(|a| {
                            if let Some(url) = author.avatar_url() {
                                a.name(author.name).icon_url(url)
                            } else {
                                a.name(author.name)
                            }
                        })
                        .title::<String>(title)
                        .description(desc)
                        .color(Color::from_rgb(73, 248, 22)),
                })
            })
            .await
        {
            error!(
                "Error sending message (channel_id: {}): {}",
                channel_id, why
            );
        }
    }
}
