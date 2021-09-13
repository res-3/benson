use chrono::Utc;
use chrono_humanize::{Accuracy, Tense};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

use crate::discord::{bot_data::check_if_privileged, state::BotState};

#[command]
pub async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get a read lock on the config data
    let data_lock = ctx.data.read().await;
    let state_arc = data_lock.get::<BotState>().unwrap().clone();
    let state = state_arc.read().await;

    // Check if the user has permission to use this command
    if check_if_privileged(&msg.member, &state.config) {
        info!("Executing status command from user: {}", msg.author);
        
        // Get the uptime
        let uptime = chrono_humanize::HumanTime::from(Utc::now() - state.startup_time)
            .to_text_en(Accuracy::Rough, Tense::Present);

        // Send the message
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "**Uptime:** {}\n**Mode:** {}\n**Host:** `{}`",
                    uptime,
                    match cfg!(debug_assertions) {
                        true => "Development",
                        false => "Production",
                    },
                    hostname::get().unwrap_or("Unknown".into()).to_str().unwrap()
                ),
            )
            .await?;
    }

    Ok(())
}
