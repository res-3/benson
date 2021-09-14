use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

use crate::discord_utils::send_message;

#[command]
pub async fn uwu(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("Executing uwu command from user: {}", msg.author);

    // Fetch text from args
    let text = args.rest();

    send_message(&uwuifier::uwuify_str_sse(text), &msg.channel_id, ctx).await;

    Ok(())
}
