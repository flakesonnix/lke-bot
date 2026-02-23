use crate::{BotState, Error};
use poise::serenity_prelude::{Reaction, ReactionType, RoleId};

pub struct ReactionRoleHandler;

impl ReactionRoleHandler {
    pub async fn handle_reaction_add(
        ctx: &poise::serenity_prelude::Context,
        reaction: &Reaction,
        data: &BotState,
    ) -> Result<(), Error> {
        if reaction.user(&ctx.http).await.map(|u| u.bot).unwrap_or(false) {
            return Ok(());
        }

        let guild_id = match reaction.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let channel_id = reaction.channel_id;
        let message_id = reaction.message_id;

        let emoji_str = Self::emoji_to_string(&reaction.emoji);

        if let Some(msg) = data.reaction_role_repo.get_message(&channel_id.to_string(), &message_id.to_string()).await? {
            let items = data.reaction_role_repo.get_message_items(msg.id).await?;
            
            for item in items {
                if let Some(rr) = data.reaction_role_repo.get_by_id(item.reaction_role_id).await? {
                    if rr.enabled && rr.emoji == emoji_str {
                        if let Ok(role_id) = rr.role_id.parse::<u64>() {
                            if let Ok(member) = guild_id.member(&ctx.http, reaction.user_id.unwrap()).await {
                                let _ = member.add_role(&ctx.http, RoleId::new(role_id)).await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn handle_reaction_remove(
        ctx: &poise::serenity_prelude::Context,
        reaction: &Reaction,
        data: &BotState,
    ) -> Result<(), Error> {
        let guild_id = match reaction.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let channel_id = reaction.channel_id;
        let message_id = reaction.message_id;

        let emoji_str = Self::emoji_to_string(&reaction.emoji);

        if let Some(msg) = data.reaction_role_repo.get_message(&channel_id.to_string(), &message_id.to_string()).await? {
            let items = data.reaction_role_repo.get_message_items(msg.id).await?;
            
            for item in items {
                if let Some(rr) = data.reaction_role_repo.get_by_id(item.reaction_role_id).await? {
                    if rr.enabled && rr.emoji == emoji_str {
                        if let Ok(role_id) = rr.role_id.parse::<u64>() {
                            if let Ok(member) = guild_id.member(&ctx.http, reaction.user_id.unwrap()).await {
                                let _ = member.remove_role(&ctx.http, RoleId::new(role_id)).await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn emoji_to_string(emoji: &ReactionType) -> String {
        match emoji {
            ReactionType::Unicode(s) => s.clone(),
            ReactionType::Custom { id, .. } => id.to_string(),
            _ => String::new(),
        }
    }
}
