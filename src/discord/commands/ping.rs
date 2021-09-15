use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

use crate::sentry_user;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);
    info!("Executing ping command from user: {}", msg.author);
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}
