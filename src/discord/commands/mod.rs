use self::bonk::*;
use self::join_vc::*;
use self::leave_vc::*;
use self::minecraft::*;
use self::ping::*;
use self::status::*;
use self::uwu::*;
use serenity::framework::standard::macros::group;

pub mod bonk;
pub mod join_vc;
pub mod leave_vc;
pub mod minecraft;
pub mod ping;
pub mod status;
pub mod uwu;

#[group]
#[commands(ping, uwu, minecraft, bonk)]
pub struct BotCommands;

#[group]
#[prefixes("benson", "bn")]
#[commands(ping, status, join_vc, fuckoff, bonk)]
pub struct BensonCommands;

#[macro_export]
macro_rules! sentry_user {
    ($msg:expr) => {
        sentry::configure_scope(|scope| {
            scope.set_user(Some(sentry::User {
                id: Some($msg.author.id.to_string()),
                email: None,
                ip_address: None,
                username: Some($msg.author.name.clone()),
                ..Default::default()
            }));
        });
    };
}
