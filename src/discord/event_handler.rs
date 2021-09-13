use chrono::{DateTime, Utc};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::{Activity, Ready},
};
use tracing::info;

use crate::config::Config;

/// The main struct that handles all discord events and contains bot state
pub struct BotEventHandler {
    startup_time: DateTime<Utc>,
    config: Config,
}

impl BotEventHandler {
    /// Construct a new `BotEventHandler`.
    pub fn new(config: &Config) -> Self {
        Self {
            startup_time: Utc::now(),
            config: (*config).clone(),
        }
    }
}

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        // Set the bot status
        ctx.set_activity(Activity::watching("over my children"))
            .await;
    }
}
