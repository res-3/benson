//! Handles automatically reacting to messages

use serenity::{
    client::Context,
    model::{
        channel::{Message, ReactionType},
        id::ChannelId,
    },
};
use tracing::info;

/// Try to auto-react to a message, or do nothing if it's not needed
pub async fn maybe_autoreact(msg: &Message, ctx: &Context, react_channel: &ChannelId) {
    // Ensure the message is in the correct channel
    if msg.channel_id == *react_channel {
        // Ensure the message is an image
        if msg.attachments.len() > 0 {
            info!("Found a message matching auto-react filter");
            // Add a reaction
            msg.react(&ctx, ReactionType::Unicode("❤️".to_string())).await.ok();
        }
    }
}
