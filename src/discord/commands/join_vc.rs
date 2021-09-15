use chrono::Utc;
use chrono_humanize::{Accuracy, Tense};
use serenity::{client::Context, framework::standard::{macros::command, Args, CommandResult}, model::{channel::Message, id::{ChannelId, GuildId}}};
use tracing::info;

use crate::{audio::join_guild_voice_channel, discord::{bot_data::check_if_privileged, state::BotState}, discord_utils::send_message, sentry_user};

#[command]
pub async fn join_vc(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);
    // Get a read lock on the config data
    let data_lock = ctx.data.read().await;
    let state_arc = data_lock.get::<BotState>().unwrap().clone();
    let state = state_arc.read().await;

    // Check if the user has permission to use this command
    if check_if_privileged(&msg.member, &state.config) {
        info!("Executing status command from user: {}", msg.author);

        // Parse the second arg
        if let Ok(channel) = args.single::<u64>() {
            // Join the voice channel
            join_guild_voice_channel(&ctx, GuildId(state.config.guild), ChannelId(channel)).await;
        } else {
            send_message("`Usage: !benson_join_vc <id>`", &msg.channel_id, &ctx).await;
        }
    }

    Ok(())
}
