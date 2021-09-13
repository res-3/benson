use std::sync::Arc;

use chrono::{DateTime, Utc};
use serenity::prelude::{RwLock, TypeMapKey};

use crate::config::Config;

/// Defines the common state for the bot
pub struct State {
    pub startup_time: DateTime<Utc>,
    pub config: Config,
}

/// Defines a wrapper for the state
pub struct BotState;

impl TypeMapKey for BotState {
    type Value = Arc<RwLock<State>>;
}
