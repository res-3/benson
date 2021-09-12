use chrono::{DateTime, Utc};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        guild::PartialMember,
        id::{ChannelId, GuildId, RoleId, UserId},
        prelude::{Activity, Ready},
    },
};
use tracing::{debug, error, info, warn};

use crate::{
    audio::{join_guild_voice_channel, leave_guild_voice_channels},
    config::Config,
};

fn validate_user(member: &PartialMember, bot_dev_role: &RoleId) -> bool {
    member.roles.contains(bot_dev_role)
        || match &member.user {
            Some(user) => user.id == UserId(375371353085444097),
            None => false,
        }
}

pub struct Handler {
    pub config: Config,
    pub start_time: DateTime<Utc>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the sender is a privileged user
        let is_privileged = match msg.member {
            Some(member) => validate_user(&member, &RoleId(self.config.bot_developer_role)),
            None => false,
        };

        // Grab the first "word" of the message in search of a command
        let mut args = msg.content.split_whitespace();
        match args.next() {
            Some(cmd) => match cmd {
                "!benson_ping" => {
                    info!(
                        "Got command benson_ping from user: {} (prived: {})",
                        msg.author.name, is_privileged
                    );
                    if is_privileged {
                        if let Err(why) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!("Bot has been running since `{}`", self.start_time),
                            )
                            .await
                        {
                            error!("Error sending message: {:?}", why);
                        }
                    }
                }
                "!benson_join_vc" => {
                    info!(
                        "Got command benson_join_vc from user: {} (prived: {})",
                        msg.author.name, is_privileged
                    );
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
                                if let Err(why) = msg
                                    .channel_id
                                    .say(
                                        &ctx.http,
                                        format!("Could not join VC with id `{}`", channel),
                                    )
                                    .await
                                {
                                    error!("Error sending message: {:?}", why);
                                }
                            }
                        } else {
                            if let Err(why) = msg
                                .channel_id
                                .say(&ctx.http, "`Usage: !benson_join_vc <id>`")
                                .await
                            {
                                error!("Error sending message: {:?}", why);
                            }
                        }
                    }
                }
                "!benson_flush" => {
                    info!(
                        "Got command benson_flush from user: {} (prived: {})",
                        msg.author.name, is_privileged
                    );
                    if is_privileged {
                        // Leave all VCs
                        leave_guild_voice_channels(&ctx, GuildId(self.config.guild)).await;

                        // Notify user
                        if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Bot has left all voice channels")
                            .await
                        {
                            error!("Error sending message: {:?}", why);
                        }
                    }
                }
                _ => {
                    debug!("Unknown command: {}", cmd);
                }
            },
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
