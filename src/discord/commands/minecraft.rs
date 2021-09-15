use async_minecraft_ping::ConnectionConfig;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

use crate::sentry_user;

#[command]
pub async fn minecraft(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_user!(msg);
    info!("Executing minecraft command from user: {}", msg.author);

    // Fetch server status
    let config = ConnectionConfig::build("142.44.143.70").with_port(25622);
    let mut connection = config.connect().await.unwrap();
    let status = connection.status().await.unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Minecraft Server Info");
                e.description("The Res3 Minecraft server IP is:\n```\n142.44.143.70:25622\n```");
                e.field("Version", status.version.name, true);
                e.field(
                    "Players",
                    format!("{}/{}", status.players.online, status.players.max),
                    true,
                );
                if let Some(sample) = status.players.sample {
                    if sample.len() > 0 {
                        e.field(
                            "Currently Online",
                            sample
                                .iter()
                                .map(|p| {
                                    format!("[{}](https://namemc.com/profile/{})", p.name, p.id)
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                            false,
                        );
                    }
                }
                e
            });
            m
        })
        .await?;

    Ok(())
}
