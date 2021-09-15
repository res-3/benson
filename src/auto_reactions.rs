//! Handles automatically reacting to messages

use rand::prelude::{SliceRandom, ThreadRng};
use serenity::{
    client::Context,
    model::{
        channel::{Message, ReactionType},
        id::ChannelId,
    },
};
use tracing::info;

use crate::{config::Config, discord_utils::send_message};

/// Try to auto-react to a message, or do nothing if it's not needed
pub async fn maybe_autoreact(msg: &Message, ctx: &Context, react_channel: &ChannelId) {
    // Ensure the message is in the correct channel
    if msg.channel_id == *react_channel {
        // Ensure the message is an image
        if msg.attachments.len() > 0 {
            info!("Found a message matching auto-react filter");
            // Add a reaction
            msg.react(&ctx, ReactionType::Unicode("❤️".to_string()))
                .await
                .ok();
        }
    }
}

/// Try to auto-correct messages containing "uwuna"
pub async fn maybe_correct_luna(msg: &Message, ctx: &Context) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "uwuna" in message content
        if msg.content.to_lowercase().contains("uwuna") {
            info!("Found a message containing uwuna");
            // Send back correction
            msg.reply_ping(&ctx.http, "I think you mean <@489817495037804545>")
                .await
                .unwrap();
        } else if msg.content.to_lowercase().contains("luna") {
            info!("Found a message containing luna");
            // Send back correction
            msg.reply_ping(&ctx.http, "I think you mean *UwUna*")
                .await
                .unwrap();
        }
    }
}

/// Try to auto-react to a message if benson is mentioned
pub async fn maybe_benson_greeting(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().contains("benson")
            && !msg.content.to_lowercase().contains("!benson")
        {
            info!("Found a message containing benson");
            msg.reply_ping(
                &ctx.http,
                config
                    .benson_responses
                    .get(rand::random::<usize>() % config.benson_responses.len())
                    .unwrap_or(&"Hello :)".to_string()),
            )
            .await
            .unwrap();
        }
    }
}
