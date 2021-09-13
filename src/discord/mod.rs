use std::{collections::HashSet, iter::FromIterator};

use serenity::{framework::StandardFramework, Client};
use songbird::{SerenityInit, Songbird};

use crate::config::Config;

use self::{
    bot_data::{get_bot_metadata, get_bot_owners},
    event_handler::BotEventHandler,
};

pub mod bot_data;
mod commands;
mod event_handler;

/// Set up and create the bot client
pub async fn init_bot_client(
    discord_token: &str,
    discord_app_id: &u64,
    config: &Config,
) -> Result<Client, serenity::Error> {
    // Get the bot metadata
    let metadata = get_bot_metadata(discord_token).await?;
    let owners = get_bot_owners(&metadata).await;

    // Create a mapping for serenity to accept the owners list
    let owners_map = HashSet::from_iter(owners);

    // Set up the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.allow_dm(true)
                .on_mention(Some(metadata.id))
                .prefix("!")
                .owners(owners_map)
        })
        .group(&commands::BOTCOMMANDS_GROUP).group(&commands::BENSONCOMMANDS_GROUP);

    // Get a voice context
    let voice = Songbird::serenity();

    // Set up the client
    Client::builder(discord_token)
        .application_id(*discord_app_id)
        .framework(framework)
        .event_handler(BotEventHandler::new(config))
        .register_songbird_with(voice)
        .await
}
