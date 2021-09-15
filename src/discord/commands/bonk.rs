use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    http::request,
    model::{
        channel::Message,
        prelude::{Activity, ActivityType, OnlineStatus},
    },
};
use tracing::info;

use crate::{bonk_count::{add_bonk, read_bonk_count}, discord_utils::send_message, sentry_user};

#[command]
pub async fn bonk(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);
    info!("Executing bonk command from user: {}", msg.author);

    // Register the bonk
    add_bonk();
    let bonk_count = read_bonk_count();

    // Respond with a message
    if bonk_count % 2 == 0 {
        send_message("No touch *benson* angy!", &msg.channel_id, &ctx).await;
    } else {
        send_message("hehe, yes, more bonks, *benson* happy", &msg.channel_id, &ctx).await;
    }

    Ok(())
}
