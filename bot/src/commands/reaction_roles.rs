use crate::{Context, Error};
use poise::serenity_prelude::{ChannelId, CreateEmbed, MessageId, ReactionType, RoleId};
use shared::{NewReactionRole, NewReactionRoleMessage, NewReactionRoleMessageItem};

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rr_create(
    ctx: Context<'_>,
    #[description = "Role to assign"] role: RoleId,
    #[description = "Emoji for the reaction"] emoji: String,
    #[description = "Optional description"] description: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    if data.reaction_role_repo.get(&guild_id.to_string(), &emoji).await?.is_some() {
        ctx.say(format!("❌ Reaction role with emoji {} already exists", emoji)).await?;
        return Ok(());
    }

    let rr = data.reaction_role_repo.create(NewReactionRole {
        guild_id: guild_id.to_string(),
        role_id: role.to_string(),
        emoji: emoji.clone(),
        description,
    }).await?;

    ctx.say(format!("✅ Created reaction role {} -> <@&{}> (ID: {})", emoji, role, rr.id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rr_delete(
    ctx: Context<'_>,
    #[description = "Reaction role ID"] id: i64,
) -> Result<(), Error> {
    let data = ctx.data();

    match data.reaction_role_repo.get_by_id(id).await? {
        Some(rr) => {
            let emoji = rr.emoji.clone();
            data.reaction_role_repo.delete(id).await?;
            ctx.say(format!("✅ Deleted reaction role {} (ID: {})", emoji, id)).await?;
        }
        None => {
            ctx.say("❌ Reaction role not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rr_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let rrs = data.reaction_role_repo.list(&guild_id.to_string()).await?;

    if rrs.is_empty() {
        ctx.say("No reaction roles configured").await?;
        return Ok(());
    }

    let mut description = String::new();
    for rr in &rrs {
        let status = if rr.enabled { "✅" } else { "❌" };
        let desc = rr.description.as_deref().unwrap_or("No description");
        description.push_str(&format!(
            "{} ID {} {} -> <@&{}> *{}\n",
            status, rr.id, rr.emoji, rr.role_id, desc
        ));
    }

    let embed = CreateEmbed::new()
        .title("🎭 Reaction Roles")
        .description(description)
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(
            format!("Total: {} reaction roles", rrs.len())
        ))
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rr_toggle(
    ctx: Context<'_>,
    #[description = "Reaction role ID"] id: i64,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let data = ctx.data();

    match data.reaction_role_repo.get_by_id(id).await? {
        Some(rr) => {
            data.reaction_role_repo.set_enabled(id, enabled).await?;
            ctx.say(format!(
                "{} Reaction role {} has been {}",
                if enabled { "✅" } else { "❌" },
                rr.emoji,
                if enabled { "enabled" } else { "disabled" }
            )).await?;
        }
        None => {
            ctx.say("❌ Reaction role not found").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rrmsg_create(
    ctx: Context<'_>,
    #[description = "Title for the message"] title: String,
    #[description = "Description"] description: Option<String>,
    #[description = "Embed color (hex)"] color: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let color_val = color.and_then(|c| u64::from_str_radix(c.trim_start_matches('#'), 16).ok());

    let mut embed = CreateEmbed::new()
        .title(&title);
    
    if let Some(desc) = &description {
        embed = embed.description(desc);
    }
    
    if let Some(c) = color_val {
        embed = embed.color(c as u32);
    } else {
        embed = embed.color(0x5865F2);
    }

    let reply = ctx.send(poise::CreateReply::default().embed(embed.clone())).await?;
    let message = reply.message().await?;

    let msg_record = data.reaction_role_repo.create_message(NewReactionRoleMessage {
        guild_id: guild_id.to_string(),
        channel_id: message.channel_id.to_string(),
        message_id: message.id.to_string(),
        title: Some(title),
        description,
        color: color_val.map(|c| c as i64),
        created_by: ctx.author().id.to_string(),
    }).await?;

    ctx.say(format!("✅ Created reaction role message (ID: {}). Use `/rrmsg_add` to add reaction roles.", msg_record.id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rrmsg_add(
    ctx: Context<'_>,
    #[description = "Message ID"] message_id: i64,
    #[description = "Reaction role ID to add"] reaction_role_id: i64,
) -> Result<(), Error> {
    let data = ctx.data();

    let msg = data.reaction_role_repo.get_message_by_id(message_id).await?
        .ok_or("Message not found")?;

    let rr = data.reaction_role_repo.get_by_id(reaction_role_id).await?
        .ok_or("Reaction role not found")?;

    data.reaction_role_repo.add_item(NewReactionRoleMessageItem {
        message_id: msg.id,
        reaction_role_id: rr.id,
    }).await?;

    let channel_id: u64 = msg.channel_id.parse()?;
    let message_id: u64 = msg.message_id.parse()?;
    
    let emoji = if rr.emoji.chars().all(|c| c.is_ascii_digit()) {
        ReactionType::Custom {
            id: rr.emoji.parse()?,
            animated: false,
            name: None,
        }
    } else {
        ReactionType::Unicode(rr.emoji.clone())
    };

    ChannelId::new(channel_id)
        .message(&ctx.serenity_context().http, MessageId::new(message_id))
        .await?
        .react(&ctx.serenity_context().http, emoji)
        .await?;

    ctx.say(format!("✅ Added {} -> <@&{}> to message {}", rr.emoji, rr.role_id, message_id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rrmsg_remove(
    ctx: Context<'_>,
    #[description = "Message ID"] message_id: i64,
    #[description = "Reaction role ID to remove"] reaction_role_id: i64,
) -> Result<(), Error> {
    let data = ctx.data();

    let msg = data.reaction_role_repo.get_message_by_id(message_id).await?
        .ok_or("Message not found")?;

    data.reaction_role_repo.remove_item(msg.id, reaction_role_id).await?;

    ctx.say(format!("✅ Removed reaction role {} from message {}", reaction_role_id, message_id)).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, required_permissions = "MANAGE_ROLES")]
pub async fn rrmsg_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a server")?;
    let data = ctx.data();

    let messages = data.reaction_role_repo.list_messages(&guild_id.to_string()).await?;

    if messages.is_empty() {
        ctx.say("No reaction role messages configured").await?;
        return Ok(());
    }

    let mut description = String::new();
    for msg in &messages {
        let items = data.reaction_role_repo.get_message_items(msg.id).await?;
        let title = msg.title.as_deref().unwrap_or("Untitled");
        description.push_str(&format!(
            "**ID {}** {} in <#{}>\n  {} roles\n",
            msg.id, title, msg.channel_id, items.len()
        ));
    }

    let embed = CreateEmbed::new()
        .title("📋 Reaction Role Messages")
        .description(description)
        .footer(poise::serenity_prelude::CreateEmbedFooter::new(
            format!("Total: {} messages", messages.len())
        ))
        .color(0x5865F2);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
