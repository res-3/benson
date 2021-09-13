use serenity::framework::standard::macros::group;
use self::ping::*;

// pub mod status;
pub mod ping;

#[group]
#[commands(ping)]
pub struct BotCommands;


#[group]
#[prefixes("benson", "bn")]
#[commands(ping)]
pub struct BensonCommands;
