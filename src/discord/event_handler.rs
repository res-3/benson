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

use crate::auto_reactions::{maybe_autoreact, maybe_benson_balls, maybe_benson_greeting, maybe_braincell_check, maybe_correct_luna, maybe_fuckle, maybe_zwspam};

use super::state::BotState;
use crate::sentry_user;

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
        sentry_user!(msg);
        // Get a read lock on the config data
        let data_lock = ctx.data.read().await;
        let state_arc = data_lock.get::<BotState>().unwrap().clone();
        let state = state_arc.read().await;

        // Handle auto-reactions
        let do_not_autoreact = maybe_benson_balls(&msg, &ctx, &state.config).await;
        if !do_not_autoreact{
            maybe_autoreact(&msg, &ctx, &ChannelId(state.config.heart_react_channel)).await;
            maybe_benson_greeting(&msg, &ctx, &state.config).await;
            maybe_correct_luna(&msg, &ctx).await;
            maybe_braincell_check(&msg, &ctx, &state.config).await;
            maybe_zwspam(&msg, &ctx, &state.config).await;
            maybe_fuckle(&msg, &ctx, &state.config).await;
        } 
    }
}
