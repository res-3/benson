use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
};
use songbird::SongbirdKey;

/// Join a voice channel and do nothing
pub async fn join_guild_voice_channel(ctx: &Context, guild: GuildId, channel: ChannelId) {
    // Get an audio manager
    let songbird_data = ctx.data.read().await;
    let manager = songbird_data
        .get::<SongbirdKey>()
        .cloned()
        .expect("Songbird Voice client placed in at initialization.");

    // Get the caller's voice channel
    let _handler = manager.join(guild, channel).await;
}

/// Leave all voice channels
pub async fn leave_guild_voice_channels(ctx: &Context, guild: GuildId) {
    // Get an audio manager
    let songbird_data = ctx.data.read().await;
    let manager = songbird_data
        .get::<SongbirdKey>()
        .cloned()
        .expect("Songbird Voice client placed in at initialization.");

    // Get the caller's voice channel
    manager.leave(guild).await.unwrap();
}
