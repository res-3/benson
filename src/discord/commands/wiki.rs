use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

use crate::{discord_utils::send_message, sentry_user};

#[command]
pub async fn wiki(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);
    info!("Executing wiki command from user: {}", msg.author);

    // Fetch text from args
    let text = args.rest();

    send_message(&format!("https://backup15.terasp.net/api/screenshot?resX=700&resY=170&outFormat=png&waitTime=100&isFullPage=false&url=https://en.wikipedia.org/wiki/{}", urlencoding::encode(text)), &msg.channel_id, ctx).await;

    Ok(())
}
