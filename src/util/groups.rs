use serenity::framework::standard::macros::group;

use crate::commands::{
    config::{general::*, moderation::*},
    dnd::dice::*,
    moderation::*,
    ticket::create::*
};

#[group]
#[commands(kick)]
pub struct Moderation;

#[group]
#[commands(setup, default_role, prefix, moderation_role)]
#[prefixes("c", "config")]
#[description = "
You can use these command to configure various things fot this bot, like the moderation roles.

Usage:
```
!config <value> <data>
!c <value> <data>
```
"]
pub struct Config;

#[group]
#[commands(create)]
#[prefixes("t", "ticket")]
#[description = "
"]
pub struct Ticket;

#[group]
#[commands(roll)]
pub struct DnD;
