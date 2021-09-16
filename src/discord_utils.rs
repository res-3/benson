//! Utilities for interfacing with discord

use serenity::{
    client::Context,
    model::{
        guild::{Guild, PartialMember},
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

// pub async fn add_user_role(member: &PartialMember, guild: &Guild, role: &RoleId, ctx: &Context) {
//     let mut user_roles = member.roles.clone();
//     user_roles.push(role.clone());
//     info!(
//         "Setting user {} roles to: {:?}",
//         member.user.map(|u| u.name).unwrap_or("Unknown".to_string()),
//         user_roles
//     );
//     guild
//         .edit_member(&ctx, msg.author.id, |m| m.roles(user_roles))
//         .await
//         .unwrap();
// }
