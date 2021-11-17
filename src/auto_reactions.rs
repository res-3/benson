//! Handles automatically reacting to messages

use serenity::model::id::{EmojiId, RoleId, UserId};
use serenity::{
    client::Context,
    model::{
        channel::{Message, ReactionType},
        id::ChannelId,
    },
};
use tracing::info;

use crate::{config::Config, discord_utils::send_message};

/// Try to auto-react to a message, or do nothing if it's not needed
pub async fn maybe_autoreact(msg: &Message, ctx: &Context, react_channel: &ChannelId) {
    // Ensure the message is in the correct channel
    if msg.channel_id == *react_channel {
        // Ensure the message is an image
        if msg.attachments.len() > 0 {
            info!("Found a message matching auto-react filter");
            // Add a reaction
            msg.react(&ctx, ReactionType::Unicode("❤️".to_string()))
                .await
                .ok();
        }
    }
}

/// Try to auto-correct messages containing "uwuna"
pub async fn maybe_correct_luna(msg: &Message, ctx: &Context) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "uwuna" in message content
        if msg.content.to_lowercase().contains("uwuna") {
            info!("Found a message containing uwuna");
            // Send back correction
            msg.reply_ping(&ctx.http, "I think you mean <@489817495037804545>")
                .await
                .unwrap();
        } else if msg.content.to_lowercase().contains("luna") {
            info!("Found a message containing luna");
            // Send back correction
            msg.reply_ping(&ctx.http, "I think you mean *UwUna*")
                .await
                .unwrap();
        }
    }
}

/// Try to auto-react to a message if benson is mentioned
pub async fn maybe_benson_greeting(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().contains("benson")
            && !msg.content.to_lowercase().contains("!benson")
        {
            info!("Found a message containing benson");
            msg.reply_ping(
                &ctx.http,
                config
                    .benson_responses
                    .get(rand::random::<usize>() % config.benson_responses.len())
                    .unwrap_or(&"Hello :)".to_string()),
            )
            .await
            .unwrap();
        }
    }
}

/// Try to handle balls commands
pub async fn maybe_benson_balls(msg: &Message, ctx: &Context, config: &Config) -> bool {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        let message = msg.content.to_lowercase();
        // Handle enable commands
        if config.benson_balls_enable_triggers.contains(&message) {
            info!("Got balls enable trigger from user: {}", msg.author);
            if let Some(member) = &msg.member {
                let mut user_roles = member.roles.clone();
                user_roles.push(RoleId(config.benson_balls_role));
                info!("Setting user {} roles to: {:?}", msg.author, user_roles);
                msg.guild(&ctx)
                    .await
                    .unwrap()
                    .edit_member(&ctx, msg.author.id, |m| m.roles(user_roles))
                    .await
                    .unwrap();
                send_message(
                    &format!("*<@{}> turns **on** their balls*", msg.author.id),
                    &msg.channel_id,
                    &ctx,
                )
                .await;
                return true;
            }
        }
        // Handle disable commands
        else if config.benson_balls_disable_triggers.contains(&message) {
            info!("Got balls disable trigger from user: {}", msg.author);
            if let Some(member) = &msg.member {
                let balls_role = RoleId(config.benson_balls_role);
                let user_roles = member
                    .roles
                    .clone()
                    .drain_filter(|x| *x != balls_role)
                    .collect::<Vec<RoleId>>();
                info!("Setting user {} roles to: {:?}", msg.author, user_roles);
                msg.guild(&ctx)
                    .await
                    .unwrap()
                    .edit_member(&ctx, msg.author.id, |m| m.roles(user_roles))
                    .await
                    .unwrap();
                send_message(
                    &format!("*<@{}> turns **off** their balls*", msg.author.id),
                    &msg.channel_id,
                    &ctx,
                )
                .await;
                return true;
            }
        }
    }
    return false;
}

/// Handle the "braincell check" word trigger
pub async fn maybe_braincell_check(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().starts_with("braincell check") {
            info!("Running Braincell Check");

            // Try to fetch a username as the third argument
            let mut args = msg.content.split_whitespace();
            args.next();
            args.next();
            let message_title = match args.next() {
                Some(x) => format!("Does {} have the braincell?", x),
                None => "Is the braincell present?".to_string(),
            };

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.content(message_title);
                    m.reactions(vec![
                        ReactionType::Unicode("✅".to_string()),
                        ReactionType::Unicode("❌".to_string()),
                    ]);
                    m
                })
                .await
                .unwrap();
        }
    }
}

/// Handle the zws spammer
pub async fn maybe_zwspam(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase() == "\u{200b}" {
            info!("Running zwspam");

            send_message("\u{200b}", &msg.channel_id, &ctx).await;
        }
    }
}

/// Handle the fuckle command
pub async fn maybe_fuckle(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().contains("fuckle") {
            info!("Running fuckle react");

            // Find the fuckle emote
            let guild_emotes = msg.guild(&ctx).await.unwrap();
            let fuckle_emote = guild_emotes
                .emojis
                .iter()
                .find(|(id, emote)| emote.name.contains("BenAhaha"))
                .unwrap();

            msg.react(
                &ctx.http,
                ReactionType::Custom {
                    animated: false,
                    id: fuckle_emote.0.clone(),
                    name: Some(fuckle_emote.1.name.clone()),
                },
            )
            .await
            .unwrap();
        }
    }
}

/// Handle the rail command
pub async fn maybe_rail(msg: &Message, ctx: &Context, config: &Config) -> bool {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().contains("rail me")
            && msg.content.to_lowercase().contains("benson")
        {
            info!("Running railing react");

            // Generate a random number to handle the responses
            let response = match rand::random::<usize>() % 10 {
                0..=6 => "No. Fuck off",
                _ => "***BENSON RAILS YOU***",
            };

            // Respond
            msg.reply_ping(&ctx.http, response).await.unwrap();
            return true;
        }
    }
    false
}

/// Handle the stfu triggers
pub async fn maybe_stfu(msg: &Message, ctx: &Context, config: &Config) -> bool {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check for "benson" in message content
        if msg.content.to_lowercase().contains("i hardly know her") {
            info!("Running stfu react");

            // Respond
            msg.reply_ping(&ctx.http, "*Shut up*").await.unwrap();
            return true;
        }
    }
    false
}

/// Handle "good bot"
pub async fn maybe_good_bot(msg: &Message, ctx: &Context, config: &Config) {
    // Do not allow the bot to respond to itself
    if !msg.author.bot {
        // Check if the message is a response to benson
        if let Some(parent) = &msg.referenced_message {
            // Check that the parent message is from benson and the reply is from evan
            if parent.author.id == ctx.cache.current_user().await.id && msg.author.id == UserId(375371353085444097) {
                // Check the message content
                if msg.content.to_lowercase().contains("good bot") {
                    info!("Running good bot react");

                    // Respond
                    msg.reply_ping(&ctx.http, "Thank you master").await.unwrap();
                }
            }
        }
        // // Check for "benson" in message content
        // if msg.content.to_lowercase().contains("i hardly know her")
        // {
        //     info!("Running stfu react");

        //     // Respond
        //     msg.reply_ping(&ctx.http, "*Shut up*").await.unwrap();
        // }
    }
}
