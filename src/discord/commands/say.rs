use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    http::request,
    model::{
        channel::Message,
        id::UserId,
        prelude::{Activity, ActivityType, OnlineStatus},
    },
};
use tracing::info;

use crate::{
    bonk_count::{add_bonk, read_bonk_count},
    discord_utils::send_message,
    sentry_user,
};

#[command]
pub async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);

    // Only evan can do this
    if msg.author.id == UserId(375371353085444097) {
        info!("Executing say command from user: {}", msg.author);

        // Grab the message contents
        let text = args.rest();
        info!("Saying: {}", text);

        // Delete the message
        msg.delete(&ctx).await?;

        // Send the message but as benson
        send_message(text, &msg.channel_id, &ctx).await;
    }

    Ok(())
}
