use serenity::{
    http::Http,
    model::{id::UserId, prelude::CurrentApplicationInfo},
};

/// Get the bot's settings & metadata
pub async fn get_bot_metadata(
    discord_token: &str,
) -> Result<CurrentApplicationInfo, serenity::Error> {
    Http::new_with_token(discord_token)
        .get_current_application_info()
        .await
}

/// Gets a list of all users set as owners in the bot developer panel
pub async fn get_bot_owners(app_info: &CurrentApplicationInfo) -> Vec<UserId> {
    let mut output = vec![app_info.owner.id];
    if let Some(team) = &app_info.team {
        team.members.iter().for_each(|member| {
            output.push(member.user.id);
        })
    }
    output
}
