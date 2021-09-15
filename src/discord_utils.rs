//! Utilities for interfacing with discord

use serenity::{
    client::Context,
    model::{
        guild::PartialMember,
        id::{ChannelId, RoleId, UserId},
    },
};
use tracing::{error, info};

/// Check if a member is allowed to run restricted commands
pub fn validate_user(member: &PartialMember, bot_dev_role: &RoleId) -> bool {
    member.roles.contains(bot_dev_role)
        || match &member.user {
            Some(user) => {
                user.id == UserId(375371353085444097) || user.id == UserId(489817495037804545)
            }
            None => false,
        }
}

/// Say a message in a channel
pub async fn send_message(text: &str, channel: &ChannelId, ctx: &Context) {
    info!("Sending message: {}", text);
    if let Err(why) = channel.say(&ctx.http, text).await {
        error!("Error sending message: {:?}", why);
    }
}
