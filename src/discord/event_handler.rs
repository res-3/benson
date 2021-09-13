use chrono::{DateTime, Utc};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::ChannelId,
        prelude::{Activity, Ready},
    },
};
use tracing::info;

use crate::auto_reactions::maybe_autoreact;

use super::state::BotState;

/// The main struct that handles all discord events
pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        // Set the bot status
        ctx.set_activity(Activity::watching("over my children"))
            .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Get a read lock on the config data
        let data_lock = ctx.data.read().await;
        let state_arc = data_lock.get::<BotState>().unwrap().clone();
        let state = state_arc.read().await;

        // Handle auto-reactions
        maybe_autoreact(&msg, &ctx, &ChannelId(state.config.heart_react_channel)).await;
    }
}