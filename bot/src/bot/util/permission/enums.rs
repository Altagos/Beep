use serenity::model::prelude::*;

#[allow(dead_code)]
pub enum PermissionOverwriteKey {
    Id,
    GuildId,
    Data(PermissionOverwriteDataKey),
}

#[allow(dead_code)]
pub enum PermissionOverwriteDataKey {
    Role,
    User,
}

#[allow(dead_code)]
pub enum PermissionOverwriteData {
    Id(u64),
    GuildId(GuildId),
    Role {
        role: RoleId,
        channel: Option<ChannelId>,
    },
    User {
        user: UserId,
        channel: Option<ChannelId>,
    },
}
