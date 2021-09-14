use serenity::framework::standard::macros::group;
use self::ping::*;
use self::status::*;
use self::join_vc::*;
use self::leave_vc::*;
use self::uwu::*;


pub mod status;
pub mod ping;
pub mod join_vc;
pub mod leave_vc;
pub mod uwu;

#[group]
#[commands(ping, uwu)]
pub struct BotCommands;


#[group]
#[prefixes("benson", "bn")]
#[commands(ping, status, join_vc, fuckoff)]
pub struct BensonCommands;
