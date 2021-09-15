#![feature(drain_filter)]

use chrono::Utc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use serenity::Client;
use songbird::{SerenityInit, Songbird};
use tracing::{error, info};

use crate::{config::Config, discord::init_bot_client};

mod audio;
mod auto_reactions;
mod bonk_count;
mod config;
mod discord;
mod discord_utils;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .help("Path to a config.json file")
                .required(true),
        )
        .get_matches();

    // Enable logging
    tracing_subscriber::fmt::init();

    // Load the config file
    let config_path = matches.value_of("config").unwrap();
    let config: Config = autojson::structify(config_path).unwrap();
    info!("Loaded config file: {}", config_path);

    // Set up the sentry error reporting client
    let _guard = sentry::init((
        config.sentry_ingest_url.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Get the discord tokens
    let discord_token = std::env::var("DISCORD_TOKEN").expect("$DISCORD_TOKEN not set");
    let discord_app_id: u64 = std::env::var("DISCORD_APP_ID")
        .expect("$DISCORD_APP_ID not set")
        .parse()
        .expect("Application ID is not valid");

    // Set up the bot client
    let mut client = init_bot_client(&discord_token, &discord_app_id, &config)
        .await
        .expect("Failed to init bot client");

    // Run the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
