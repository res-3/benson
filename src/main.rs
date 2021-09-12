use chrono::Utc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use serenity::Client;
use songbird::{SerenityInit, Songbird};
use tracing::{error, info};

use crate::config::Config;

mod audio;
mod command_handler;
mod config;

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

    // Get the discord tokens
    let discord_token = std::env::var("DISCORD_TOKEN").expect("$DISCORD_TOKEN not set");
    let discord_app_id: u64 = std::env::var("DISCORD_APP_ID")
        .expect("$DISCORD_APP_ID not set")
        .parse()
        .expect("Application ID is not valid");

    // Get a voice context
    let voice = Songbird::serenity();

    // Build the discord bot
    let mut client = Client::builder(&discord_token)
        .event_handler(command_handler::Handler {
            config,
            start_time: Utc::now(),
        })
        .application_id(discord_app_id)
        .register_songbird_with(voice)
        .await
        .expect("Err creating client");

    // Run the bot
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
