use serenity::framework::standard::macros::group;

use crate::commands::{config::general::*, dnd::dice::*, moderation::*};

#[group]
#[commands(kick)]
pub struct Moderation;

#[group]
#[commands(setup, default_role)]
#[prefixes("c", "config")]
#[description = "
You can use these command to configure various things fot this command, like the moderation roles.

Usage:
```
!config <value> <data>
!c <value> <data>
```
"]
pub struct Config;

#[group]
#[commands(roll)]
pub struct DnD;
