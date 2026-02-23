use crate::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, RoleId, ChannelId};
use shared::UpdateModerationSettings;

/// Show current auto-mod settings
#[poise::command(slash_command, prefix_command)]
pub async fn automod_settings(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let settings = data.moderation_repo.get_settings().await?;

    let status = if settings.enabled { "✅ Enabled" } else { "❌ Disabled" };
    let log_channel = settings.log_channel_id
        .map(|id| format!("<#{}>", id))
        .unwrap_or_else(|| "Not set".to_string());
    let mute_role = settings.mute_role_id
        .map(|id| format!("<@&{}>", id))
        .unwrap_or_else(|| "Not set".to_string());

    let embed = CreateEmbed::new()
        .title("🛡️ Auto-Moderation Settings")
        .field("Status", status, true)
        .field("Log Channel", log_channel, true)
        .field("Mute Role", mute_role, true)
        .field("Check Bad Words", if settings.check_bad_words { "✅" } else { "❌" }, true)
        .field("Check Bad Names", if settings.check_bad_names { "✅" } else { "❌" }, true)
        .field("Check NSFW Avatars", if settings.check_nsfw_avatars { "✅" } else { "❌" }, true)
        .field("Warn Threshold", settings.warn_threshold.to_string(), true)
        .field("Auto Mute", if settings.auto_mute { "✅" } else { "❌" }, true)
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Enable or disable auto-moderation
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn automod_toggle(
    ctx: Context<'_>,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let data = ctx.data();
    let current = data.moderation_repo.get_settings().await?;

    let update = UpdateModerationSettings {
        enabled,
        check_bad_words: current.check_bad_words,
        check_bad_names: current.check_bad_names,
        check_nsfw_avatars: current.check_nsfw_avatars,
        log_channel_id: current.log_channel_id,
        mute_role_id: current.mute_role_id,
        warn_threshold: current.warn_threshold,
        auto_mute: current.auto_mute,
        language: current.language,
    };

    data.moderation_repo.update_settings(update).await?;

    ctx.say(if enabled {
        "✅ Auto-moderation enabled"
    } else {
        "❌ Auto-moderation disabled"
    }).await?;

    Ok(())
}

/// Configure auto-moderation filters
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn automod_filters(
    ctx: Context<'_>,
    #[description = "Check for bad words"] check_bad_words: Option<bool>,
    #[description = "Check for bad usernames"] check_bad_names: Option<bool>,
    #[description = "Check for NSFW avatars"] check_nsfw_avatars: Option<bool>,
) -> Result<(), Error> {
    let data = ctx.data();
    let current = data.moderation_repo.get_settings().await?;

    let update = UpdateModerationSettings {
        enabled: current.enabled,
        check_bad_words: check_bad_words.unwrap_or(current.check_bad_words),
        check_bad_names: check_bad_names.unwrap_or(current.check_bad_names),
        check_nsfw_avatars: check_nsfw_avatars.unwrap_or(current.check_nsfw_avatars),
        log_channel_id: current.log_channel_id,
        mute_role_id: current.mute_role_id,
        warn_threshold: current.warn_threshold,
        auto_mute: current.auto_mute,
        language: current.language,
    };

    data.moderation_repo.update_settings(update).await?;

    ctx.say("✅ Auto-moderation filters updated").await?;

    Ok(())
}

/// Set the log channel for auto-mod actions
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn automod_logchannel(
    ctx: Context<'_>,
    #[description = "Channel for moderation logs"] channel: Option<ChannelId>,
) -> Result<(), Error> {
    let data = ctx.data();
    let current = data.moderation_repo.get_settings().await?;

    let update = UpdateModerationSettings {
        enabled: current.enabled,
        check_bad_words: current.check_bad_words,
        check_bad_names: current.check_bad_names,
        check_nsfw_avatars: current.check_nsfw_avatars,
        log_channel_id: channel.map(|c| c.to_string()),
        mute_role_id: current.mute_role_id,
        warn_threshold: current.warn_threshold,
        auto_mute: current.auto_mute,
        language: current.language,
    };

    data.moderation_repo.update_settings(update).await?;

    if let Some(ch) = channel {
        ctx.say(format!("✅ Auto-mod log channel set to <#{}>", ch)).await?;
    } else {
        ctx.say("✅ Auto-mod log channel cleared").await?;
    }

    Ok(())
}

/// Set the mute role for auto-moderation
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn automod_muterole(
    ctx: Context<'_>,
    #[description = "Role to assign when muted"] role: Option<RoleId>,
) -> Result<(), Error> {
    let data = ctx.data();
    let current = data.moderation_repo.get_settings().await?;

    let update = UpdateModerationSettings {
        enabled: current.enabled,
        check_bad_words: current.check_bad_words,
        check_bad_names: current.check_bad_names,
        check_nsfw_avatars: current.check_nsfw_avatars,
        log_channel_id: current.log_channel_id,
        mute_role_id: role.map(|r| r.to_string()),
        warn_threshold: current.warn_threshold,
        auto_mute: current.auto_mute,
        language: current.language,
    };

    data.moderation_repo.update_settings(update).await?;

    if let Some(r) = role {
        ctx.say(format!("✅ Mute role set to <@&{}>", r)).await?;
    } else {
        ctx.say("✅ Mute role cleared").await?;
    }

    Ok(())
}

/// Configure warning threshold and auto-mute
#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn automod_warnings(
    ctx: Context<'_>,
    #[description = "Warnings before auto-mute"] threshold: Option<i64>,
    #[description = "Enable auto-mute at threshold"] auto_mute: Option<bool>,
) -> Result<(), Error> {
    let data = ctx.data();
    let current = data.moderation_repo.get_settings().await?;

    let update = UpdateModerationSettings {
        enabled: current.enabled,
        check_bad_words: current.check_bad_words,
        check_bad_names: current.check_bad_names,
        check_nsfw_avatars: current.check_nsfw_avatars,
        log_channel_id: current.log_channel_id,
        mute_role_id: current.mute_role_id,
        warn_threshold: threshold.unwrap_or(current.warn_threshold),
        auto_mute: auto_mute.unwrap_or(current.auto_mute),
        language: current.language,
    };

    let auto_mute_str = if update.auto_mute { "enabled" } else { "disabled" };
    let warn_threshold = update.warn_threshold;

    data.moderation_repo.update_settings(update).await?;

    ctx.say(format!(
        "✅ Warning settings updated: threshold = {}, auto-mute = {}",
        warn_threshold, auto_mute_str
    )).await?;

    Ok(())
}

/// Check a user's warnings
#[poise::command(slash_command, prefix_command)]
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "User to check warnings for"] user: Option<poise::serenity_prelude::UserId>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let target_user = user.unwrap_or_else(|| ctx.author().id);

    let data = ctx.data();
    let warnings = data.moderation_repo.get_warnings(
        &guild_id.to_string(),
        &target_user.to_string(),
    ).await?;

    if warnings.is_empty() {
        ctx.say(format!("<@{}> has no warnings", target_user)).await?;
        return Ok(());
    }

    let mut description = String::new();
    for w in &warnings {
        description.push_str(&format!(
            "• {} - *{}* (by <@{}>)\n",
            w.reason,
            w.created_at.split('T').next().unwrap_or(&w.created_at),
            w.moderator_id
        ));
    }

    let embed = CreateEmbed::new()
        .title(format!("⚠️ Warnings for <@{}>", target_user))
        .description(description)
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(
            format!("Total: {} warnings", warnings.len())
        ))
        .color(0xFFCC00);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Warn a user (adds a warning to the database)
#[poise::command(slash_command, prefix_command, required_permissions = "MODERATE_MEMBERS")]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "User to warn"] user: poise::serenity_prelude::UserId,
    #[description = "Reason for warning"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let data = ctx.data();
    let _warning = data.moderation_repo.add_warning(shared::NewModerationWarning {
        guild_id: guild_id.to_string(),
        user_id: user.to_string(),
        reason: reason.clone(),
        moderator_id: ctx.author().id.to_string(),
    }).await?;

    let count = data.moderation_repo.get_warning_count(
        &guild_id.to_string(),
        &user.to_string(),
    ).await?;

    ctx.say(format!(
        "⚠️ <@{}> has been warned for: {}\nThey now have {} warning(s)",
        user, reason, count
    )).await?;

    Ok(())
}
