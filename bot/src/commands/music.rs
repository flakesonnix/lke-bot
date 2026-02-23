use crate::{Context, Error};
use poise::serenity_prelude::CreateEmbed;
use songbird::input::{AuxMetadata, YoutubeDl};

static HTTP_CLIENT: once_cell::sync::Lazy<reqwest::Client> = once_cell::sync::Lazy::new(|| {
    reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .expect("Failed to create HTTP client")
});

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song URL or search query"] query: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    let channel_id = {
        let guild = &ctx.guild().unwrap();
        let voice_channel = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id);

        match voice_channel {
            Some(c) => Some(c),
            None => None,
        }
    };

    let channel_id = match channel_id {
        Some(c) => c,
        None => {
            ctx.say("❌ You need to be in a voice channel!").await?;
            return Ok(());
        }
    };

    if manager.get(guild_id).is_none() {
        manager.join(guild_id, channel_id).await?;
    }

    let handler = manager.get(guild_id).unwrap();
    let mut handler = handler.lock().await;

    let source = YoutubeDl::new(HTTP_CLIENT.clone(), query.clone());
    let _track = handler.play_input(source.into());
    
    let title = query.clone();
    
    let embed = CreateEmbed::new()
        .title("🎵 Added to Queue")
        .description(format!("**{}**", title))
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    let data = ctx.data();
    let _ = data.music_repo.record_play(shared::NewMusicStat {
        guild_id: guild_id.to_string(),
        track_id: query.clone(),
        title,
        artist: None,
        source: "youtube".to_string(),
        duration_seconds: None,
        requested_by: Some(ctx.author().id.to_string()),
    }).await;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        handler.queue().skip()?;
        ctx.say("⏭️ Skipped!").await?;
    } else {
        ctx.say("❌ Not playing anything!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        handler.queue().stop();
        ctx.say("⏹️ Stopped!").await?;
    } else {
        ctx.say("❌ Not playing anything!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        if handler.queue().pause().is_ok() {
            ctx.say("⏸️ Paused!").await?;
        } else {
            ctx.say("❌ Nothing to pause!").await?;
        }
    } else {
        ctx.say("❌ Not playing anything!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        if handler.queue().resume().is_ok() {
            ctx.say("▶️ Resumed!").await?;
        } else {
            ctx.say("❌ Nothing to resume!").await?;
        }
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        let queue = handler.queue().current_queue();

        if queue.is_empty() {
            ctx.say("📭 Queue is empty!").await?;
            return Ok(());
        }

        let mut description = String::new();
        for (i, track) in queue.iter().take(10).enumerate() {
            let data = track.data::<AuxMetadata>();
            let title = data.title.as_deref().unwrap_or("Unknown");
            if i == 0 {
                description.push_str(&format!("🎵 **Now Playing:** {}\n", title));
            } else {
                description.push_str(&format!("{}. {}\n", i, title));
            }
        }

        if queue.len() > 10 {
            description.push_str(&format!("\n... and {} more", queue.len() - 10));
        }

        let embed = CreateEmbed::new()
            .title("📋 Queue")
            .description(description)
            .footer(poise::serenity_prelude::CreateEmbedFooter::new(
                format!("{} tracks", queue.len())
            ))
            .color(0x5865F2);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn volume(
    ctx: Context<'_>,
    #[description = "Volume level (0-100)"] level: Option<u8>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        
        if let Some(vol) = level {
            let vol = vol.min(100) as f32 / 100.0;
            if let Some(track) = handler.queue().current() {
                track.set_volume(vol)?;
            }
            ctx.say(format!("🔊 Volume set to {}%", (vol * 100.0) as u8)).await?;
        } else {
            ctx.say("🔊 Use `/volume <0-100>` to set volume").await?;
        }
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
        ctx.say("👋 Left the voice channel!").await?;
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn np(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        
        if let Some(track) = handler.queue().current() {
            let data = track.data::<AuxMetadata>();
            let title = data.title.as_deref().unwrap_or("Unknown");
            let artist = data.artist.as_deref().unwrap_or("Unknown Artist");
            let duration = data.duration.map(|d| format!("{:?}", d)).unwrap_or("Unknown".to_string());
            let source = data.source_url.as_deref().unwrap_or("Unknown");

            let embed = CreateEmbed::new()
                .title("🎵 Now Playing")
                .description(format!("**{}**\nBy: {}", title, artist))
                .field("Duration", duration, true)
                .field("Source", source, true)
                .color(0x5865F2);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        } else {
            ctx.say("❌ Nothing playing!").await?;
        }
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn playlist_save(
    ctx: Context<'_>,
    #[description = "Playlist name"] name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        let queue = handler.queue().current_queue();

        if queue.is_empty() {
            ctx.say("❌ Queue is empty!").await?;
            return Ok(());
        }

        let tracks: Vec<shared::QueuedTrack> = queue
            .iter()
            .map(|t| {
                let m = t.data::<AuxMetadata>();
                shared::QueuedTrack {
                    track_id: m.source_url.clone().unwrap_or_default(),
                    title: m.title.clone().unwrap_or_default(),
                    artist: m.artist.clone(),
                    source: "youtube".to_string(),
                    url: m.source_url.clone().unwrap_or_default(),
                    duration_seconds: m.duration.map(|d| d.as_secs() as i64),
                    requested_by: String::new(),
                    thumbnail_url: m.thumbnail.clone(),
                }
            })
            .collect();

        let tracks_json = serde_json::to_string(&tracks)?;
        let data = ctx.data();

        data.music_repo.create_playlist(shared::NewSavedPlaylist {
            guild_id: guild_id.to_string(),
            name: name.clone(),
            tracks: tracks_json,
            created_by: ctx.author().id.to_string(),
        }).await?;

        ctx.say(format!("✅ Saved playlist **{}** with {} tracks!", name, tracks.len())).await?;
    } else {
        ctx.say("❌ Not in a voice channel!").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn playlist_load(
    ctx: Context<'_>,
    #[description = "Playlist name"] name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let playlist = data.music_repo.get_playlist(&guild_id.to_string(), &name).await?
        .ok_or("Playlist not found")?;

    let tracks: Vec<shared::QueuedTrack> = serde_json::from_str(&playlist.tracks)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voice client not initialized");

    let channel_id = {
        let guild = &ctx.guild().unwrap();
        guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id)
            .ok_or("You need to be in a voice channel!")?
    };

    if manager.get(guild_id).is_none() {
        manager.join(guild_id, channel_id).await?;
    }

    let handler = manager.get(guild_id).unwrap();
    let mut handler = handler.lock().await;

    let mut added = 0;
    for track in &tracks {
        if !track.url.is_empty() {
            let source = YoutubeDl::new(HTTP_CLIENT.clone(), track.url.clone());
            handler.play_input(source.into());
            added += 1;
        }
    }

    ctx.say(format!("✅ Loaded **{}** with {} tracks!", name, added)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn playlist_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let playlists = data.music_repo.list_playlists(&guild_id.to_string()).await?;

    if playlists.is_empty() {
        ctx.say("📭 No saved playlists!").await?;
        return Ok(());
    }

    let mut description = String::new();
    for pl in &playlists {
        let tracks: Vec<shared::QueuedTrack> = serde_json::from_str(&pl.tracks).unwrap_or_default();
        description.push_str(&format!("• **{}** - {} tracks\n", pl.name, tracks.len()));
    }

    let embed = CreateEmbed::new()
        .title("📁 Saved Playlists")
        .description(description)
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn playlist_delete(
    ctx: Context<'_>,
    #[description = "Playlist name"] name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    data.music_repo.delete_playlist(&guild_id.to_string(), &name).await?;

    ctx.say(format!("✅ Deleted playlist **{}**", name)).await?;

    Ok(())
}
