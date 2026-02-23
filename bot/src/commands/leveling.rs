use crate::{Context, Error};
use poise::serenity_prelude::{CreateEmbed, Mentionable, UserId};

fn xp_for_level(level: i64) -> i64 {
    let mut xp_needed = 100i64;
    let mut total = 0i64;
    
    for _ in 1..level {
        total += xp_needed;
        xp_needed = (xp_needed as f64 * 1.5) as i64;
    }
    
    total
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en", "Show your or another user's rank and XP")
)]
pub async fn rank(
    ctx: Context<'_>,
    #[description = "User to check rank for"] user: Option<UserId>,
) -> Result<(), Error> {
    let target_user = user.unwrap_or_else(|| ctx.author().id);
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let data = ctx.data();
    let user_level = data.level_repo.get_or_create_user_level(
        &guild_id.to_string(),
        &target_user.to_string(),
    ).await?;

    let xp_for_next = xp_for_level(user_level.level + 1);
    let progress = user_level.xp as f64 / xp_for_next as f64 * 100.0;

    let embed = CreateEmbed::new()
        .title(format!("{}'s Rank", ctx.author().name))
        .field("Level", user_level.level.to_string(), true)
        .field("XP", format!("{} / {}", user_level.xp, xp_for_next), true)
        .field("Total XP", user_level.total_xp.to_string(), true)
        .field("Progress", format!("{:.1}%", progress), true)
        .field("Voice Time", format!("{} min", user_level.voice_minutes), true)
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en", "Show the server XP leaderboard")
)]
pub async fn leaderboard(
    ctx: Context<'_>,
    #[description = "Page number"] page: Option<i64>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let page = page.unwrap_or(1).max(1);
    let limit = 10;
    let offset = (page - 1) * limit;

    let data = ctx.data();
    let leaderboard = data.level_repo.get_leaderboard(
        &guild_id.to_string(),
        limit + offset,
    ).await?;

    let entries: Vec<_> = leaderboard.iter()
        .skip(offset as usize)
        .take(limit as usize)
        .enumerate()
        .collect();

    let mut description = String::new();
    
    for (idx, level) in entries {
        let rank = offset as usize + idx + 1;
        let medal = match rank {
            1 => "🥇",
            2 => "🥈", 
            3 => "🥉",
            _ => &format!("#{}", rank),
        };
        
        description.push_str(&format!(
            "{} <@{}> - Level {} ({} XP)\n",
            medal, level.user_id, level.level, level.total_xp
        ));
    }

    if description.is_empty() {
        description = "No users found on this page.".to_string();
    }

    let embed = CreateEmbed::new()
        .title(format!("🏆 Leaderboard (Page {})", page))
        .description(description)
        .color(0xFFD700);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    description_localized("en", "Set a user's XP (admin only)")
)]
pub async fn setxp(
    ctx: Context<'_>,
    #[description = "User to set XP for"] user: UserId,
    #[description = "Amount of XP"] amount: i64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let data = ctx.data();
    let user_level = data.level_repo.set_xp(
        &guild_id.to_string(),
        &user.to_string(),
        amount,
    ).await?;

    ctx.say(format!(
        "Set {}'s XP to {}. They are now level {}.",
        user.mention(), amount, user_level.level
    )).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_GUILD",
    description_localized("en", "Add XP to a user (admin only)")
)]
pub async fn addxp(
    ctx: Context<'_>,
    #[description = "User to add XP to"] user: UserId,
    #[description = "Amount of XP to add"] amount: i64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;

    let data = ctx.data();
    let current = data.level_repo.get_or_create_user_level(
        &guild_id.to_string(),
        &user.to_string(),
    ).await?;

    let user_level = data.level_repo.set_xp(
        &guild_id.to_string(),
        &user.to_string(),
        current.total_xp + amount,
    ).await?;

    ctx.say(format!(
        "Added {} XP to {}. They are now level {} with {} total XP.",
        amount, user.mention(), user_level.level, user_level.total_xp
    )).await?;

    Ok(())
}
