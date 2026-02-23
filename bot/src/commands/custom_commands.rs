use crate::{Context, Error};
use poise::serenity_prelude::CreateEmbed;
use shared::{NewCustomCommand, NewAutoResponse, UpdateCustomCommand, UpdateAutoResponse};

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn cmd_create(
    ctx: Context<'_>,
    #[description = "Command name (without !)"] name: String,
    #[description = "Response text"] response: String,
    #[description = "Optional description"] description: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let name = name.to_lowercase().replace('!', "");

    if data.custom_cmd_repo.get(&guild_id.to_string(), &name).await?.is_some() {
        ctx.say(format!("❌ Command `!{}` already exists", name)).await?;
        return Ok(());
    }

    let cmd = data.custom_cmd_repo.create(NewCustomCommand {
        guild_id: guild_id.to_string(),
        name: name.clone(),
        description,
        response,
        embed_title: None,
        embed_description: None,
        embed_color: None,
        embed_image_url: None,
        embed_thumbnail_url: None,
        created_by: ctx.author().id.to_string(),
    }).await?;

    ctx.say(format!("✅ Created command `!{}` (ID: {})", cmd.name, cmd.id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn cmd_delete(
    ctx: Context<'_>,
    #[description = "Command name or ID"] command: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let cmd = if let Ok(id) = command.parse::<i64>() {
        data.custom_cmd_repo.get_by_id(id).await?
    } else {
        let name = command.to_lowercase().replace('!', "");
        data.custom_cmd_repo.get(&guild_id.to_string(), &name).await?
    };

    match cmd {
        Some(c) => {
            data.custom_cmd_repo.delete(c.id).await?;
            ctx.say(format!("✅ Deleted command `!{}`", c.name)).await?;
        }
        None => {
            ctx.say("❌ Command not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn cmd_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let commands = data.custom_cmd_repo.list(&guild_id.to_string()).await?;

    if commands.is_empty() {
        ctx.say("No custom commands configured").await?;
        return Ok(());
    }

    let mut description = String::new();
    for cmd in &commands {
        let status = if cmd.enabled { "✅" } else { "❌" };
        description.push_str(&format!(
            "{} `!{}` - {} (ID: {})\n",
            status,
            cmd.name,
            cmd.description.as_deref().unwrap_or(&cmd.response).chars().take(50).collect::<String>(),
            cmd.id
        ));
    }

    let embed = CreateEmbed::new()
        .title("📝 Custom Commands")
        .description(description)
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(
            format!("Total: {} commands", commands.len())
        ))
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn cmd_edit(
    ctx: Context<'_>,
    #[description = "Command name or ID"] command: String,
    #[description = "New response"] response: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let cmd = if let Ok(id) = command.parse::<i64>() {
        data.custom_cmd_repo.get_by_id(id).await?
    } else {
        let name = command.to_lowercase().replace('!', "");
        data.custom_cmd_repo.get(&guild_id.to_string(), &name).await?
    };

    match cmd {
        Some(c) => {
            data.custom_cmd_repo.update(c.id, UpdateCustomCommand {
                description: c.description,
                response,
                embed_title: c.embed_title,
                embed_description: c.embed_description,
                embed_color: c.embed_color,
                embed_image_url: c.embed_image_url,
                embed_thumbnail_url: c.embed_thumbnail_url,
                enabled: c.enabled,
                cooldown_seconds: c.cooldown_seconds,
                require_permissions: c.require_permissions,
            }).await?;
            ctx.say(format!("✅ Updated command `!{}`", c.name)).await?;
        }
        None => {
            ctx.say("❌ Command not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn cmd_toggle(
    ctx: Context<'_>,
    #[description = "Command name or ID"] command: String,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let cmd = if let Ok(id) = command.parse::<i64>() {
        data.custom_cmd_repo.get_by_id(id).await?
    } else {
        let name = command.to_lowercase().replace('!', "");
        data.custom_cmd_repo.get(&guild_id.to_string(), &name).await?
    };

    match cmd {
        Some(c) => {
            data.custom_cmd_repo.update(c.id, UpdateCustomCommand {
                description: c.description,
                response: c.response,
                embed_title: c.embed_title,
                embed_description: c.embed_description,
                embed_color: c.embed_color,
                embed_image_url: c.embed_image_url,
                embed_thumbnail_url: c.embed_thumbnail_url,
                enabled,
                cooldown_seconds: c.cooldown_seconds,
                require_permissions: c.require_permissions,
            }).await?;
            ctx.say(format!(
                "{} Command `!{}` has been {}",
                if enabled { "✅" } else { "❌" },
                c.name,
                if enabled { "enabled" } else { "disabled" }
            )).await?;
        }
        None => {
            ctx.say("❌ Command not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn autoresp_add(
    ctx: Context<'_>,
    #[description = "Trigger type: contains, starts_with, ends_with, exact, regex"] trigger_type: String,
    #[description = "Pattern to match"] trigger_pattern: String,
    #[description = "Response text"] response: String,
    #[description = "Case sensitive matching"] case_sensitive: Option<bool>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let valid_types = ["contains", "starts_with", "ends_with", "exact", "regex"];
    let trigger_type = trigger_type.to_lowercase();
    
    if !valid_types.contains(&trigger_type.as_str()) {
        ctx.say(format!("❌ Invalid trigger type. Valid types: {}", valid_types.join(", "))).await?;
        return Ok(());
    }

    let auto_resp = data.auto_resp_repo.create(NewAutoResponse {
        guild_id: guild_id.to_string(),
        trigger_type,
        trigger_pattern,
        response,
        response_type: "text".to_string(),
        embed_title: None,
        embed_description: None,
        embed_color: None,
        created_by: ctx.author().id.to_string(),
        case_sensitive: case_sensitive.unwrap_or(false),
    }).await?;

    ctx.say(format!("✅ Created auto-response (ID: {})", auto_resp.id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn autoresp_delete(
    ctx: Context<'_>,
    #[description = "Auto-response ID"] id: i64,
) -> Result<(), Error> {
    let data = ctx.data();

    match data.auto_resp_repo.get_by_id(id).await? {
        Some(_) => {
            data.auto_resp_repo.delete(id).await?;
            ctx.say(format!("✅ Deleted auto-response {}", id)).await?;
        }
        None => {
            ctx.say("❌ Auto-response not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn autoresp_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let responses = data.auto_resp_repo.list(&guild_id.to_string()).await?;

    if responses.is_empty() {
        ctx.say("No auto-responses configured").await?;
        return Ok(());
    }

    let mut description = String::new();
    for resp in &responses {
        let status = if resp.enabled { "✅" } else { "❌" };
        let pattern: String = resp.trigger_pattern.chars().take(30).collect();
        description.push_str(&format!(
            "{} ID {} [{}] `{}` -> {}\n",
            status,
            resp.id,
            resp.trigger_type,
            pattern,
            resp.response.chars().take(30).collect::<String>()
        ));
    }

    let embed = CreateEmbed::new()
        .title("🔔 Auto-Responses")
        .description(description)
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(
            format!("Total: {} responses", responses.len())
        ))
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_GUILD")]
pub async fn autoresp_toggle(
    ctx: Context<'_>,
    #[description = "Auto-response ID"] id: i64,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let data = ctx.data();

    match data.auto_resp_repo.get_by_id(id).await? {
        Some(r) => {
            data.auto_resp_repo.update(id, UpdateAutoResponse {
                trigger_type: r.trigger_type,
                trigger_pattern: r.trigger_pattern,
                response: r.response,
                response_type: r.response_type,
                embed_title: r.embed_title,
                embed_description: r.embed_description,
                embed_color: r.embed_color,
                enabled,
                case_sensitive: r.case_sensitive,
                cooldown_seconds: r.cooldown_seconds,
            }).await?;
            ctx.say(format!(
                "{} Auto-response {} has been {}",
                if enabled { "✅" } else { "❌" },
                id,
                if enabled { "enabled" } else { "disabled" }
            )).await?;
        }
        None => {
            ctx.say("❌ Auto-response not found").await?;
        }
    }

    Ok(())
}
