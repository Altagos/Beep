use crate::util::permission::enums::{PermissionOverwriteData, PermissionOverwriteKey};
use serenity::model::prelude::*;

#[allow(dead_code)]
pub enum DatabaseCollections {
    GuildConfig {
        id: u64,
        key: GuildConfigKey,
    },
    PermissionOverwrites {
        id: u64,
        key: PermissionOverwriteKey,
    },
    Tickets {
        id: u64,
        key: TicketKey,
    },
}

#[allow(dead_code)]
pub enum GuildConfigKey {
    Id,
    DefaultRole,
    Prefix,
    ModerationRole,
    TicketCategory,
}

#[allow(dead_code)]
pub enum GuildConfigData {
    Id(u64),
    DefaultRole(RoleId),
    Prefix(String),
    ModerationRole(RoleId),
    TicketCategory(ChannelId),
}

#[allow(dead_code)]
pub enum TicketKey {
    Id,
    Guild,
    Channel,
    Author,
    Title,
    Description,
}

#[allow(dead_code)]
pub enum TicketData {
    Id(u64),
    Guild(GuildId),
    Channel(ChannelId),
    Author(UserId),
    Title(String),
    Description(String),
}

#[allow(dead_code)]
pub enum StoreResult {
    Guild(GuildConfigData),
    PermissionOverwrite(PermissionOverwriteData),
    Ticket(TicketData),
    None,
}
