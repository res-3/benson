use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};
use tracing::info;

use crate::{audio::{join_guild_voice_channel, leave_guild_voice_channels}, discord::{bot_data::check_if_privileged, state::BotState}, discord_utils::send_message};

#[command]
pub async fn fuckoff(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get a read lock on the config data
    let data_lock = ctx.data.read().await;
    let state_arc = data_lock.get::<BotState>().unwrap().clone();
    let state = state_arc.read().await;

    // Check if the user has permission to use this command
    if check_if_privileged(&msg.member, &state.config) {
        info!("Executing status command from user: {}", msg.author);

        // Leave all VCs
        leave_guild_voice_channels(&ctx, GuildId(state.config.guild)).await;

        // Notify user
        send_message("Bot has left all voice channels", &msg.channel_id, &ctx).await;
    }

    Ok(())
}
