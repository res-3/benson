use chrono::{DateTime, Utc};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::{ChannelId, GuildId, RoleId},
        prelude::{Activity, Ready},
    },
};
use tracing::{debug, info, warn};

use crate::{
    audio::{join_guild_voice_channel, leave_guild_voice_channels},
    auto_reactions::maybe_autoreact,
    config::Config,
    discord_utils::{send_message, validate_user},
};

pub struct Handler {
    pub config: Config,
    pub start_time: DateTime<Utc>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Handle auto-reactions
        maybe_autoreact(&msg, &ctx, &ChannelId(self.config.heart_react_channel)).await;

        // Check if the sender is a privileged user
        let is_privileged = match msg.member {
            Some(member) => validate_user(&member, &RoleId(self.config.bot_developer_role)),
            None => false,
        };

        // Grab the first "word" of the message in search of a command
        let mut args = msg.content.split_whitespace();
        match args.next() {
            Some(cmd) => {
                if cmd.starts_with("!") {
                    // Log the command
                    info!(
                        "Got command {} from user: {} (prived: {})",
                        cmd, msg.author.name, is_privileged
                    );

                    match cmd {
                        // Ping command
                        // This command responds with a message containing the bot uptime
                        "!benson_ping" => {
                            if is_privileged {
                                send_message(
                                    &format!("Bot has been running since `{}`", self.start_time),
                                    &msg.channel_id,
                                    &ctx,
                                )
                                .await;
                            }
                        }
                        // Join command
                        // This command joins a specific voice channel
                        "!benson_join_vc" | "~bvc" => {
                            if is_privileged {
                                // Parse the second arg
                                if let Some(channel) = args.next() {
                                    // Parse the channel
                                    if let Ok(channel) = channel.parse::<u64>() {
                                        // Join the voice channel
                                        join_guild_voice_channel(
                                            &ctx,
                                            GuildId(self.config.guild),
                                            ChannelId(channel),
                                        )
                                        .await;
                                    } else {
                                        send_message(
                                            &format!("Could not join VC with id `{}`", channel),
                                            &msg.channel_id,
                                            &ctx,
                                        )
                                        .await;
                                    }
                                } else {
                                    send_message(
                                        "`Usage: !benson_join_vc <id>`",
                                        &msg.channel_id,
                                        &ctx,
                                    )
                                    .await;
                                }
                            }
                        }
                        // Leave command
                        // This command leaves all voice channels
                        "!benson_flush" | "!benson_fuckoff" | "!benson_leave" => {
                            if is_privileged {
                                // Leave all VCs
                                leave_guild_voice_channels(&ctx, GuildId(self.config.guild)).await;

                                // Notify user
                                send_message(
                                    "Bot has left all voice channels",
                                    &msg.channel_id,
                                    &ctx,
                                )
                                .await;
                            }
                        }
                        _ => {
                            debug!("Unknown command: {}", cmd);
                        }
                    }
                }
            }
            None => {
                warn!("Message called without data");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Set the bot status
        ctx.set_activity(Activity::watching("over art kids")).await;
    }
}
