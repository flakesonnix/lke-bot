use crate::Error;
use poise::serenity_prelude::{Message, GuildId, UserId};

pub struct LevelingHandler;

impl LevelingHandler {
    pub async fn handle_message(
        ctx: &poise::serenity_prelude::Context,
        msg: &Message,
        data: &crate::BotState,
    ) -> Result<(), Error> {
        if msg.author.bot {
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let user_id = msg.author.id;

        let settings = data.level_repo.get_settings(&guild_id.to_string()).await?;
        
        if !settings.enabled {
            return Ok(());
        }

        let cooldown_passed = data.level_repo.check_cooldown(
            &guild_id.to_string(),
            &user_id.to_string(),
            settings.cooldown_seconds,
        ).await?;

        if !cooldown_passed {
            return Ok(());
        }

        let multiplier = data.level_repo.get_multiplier(
            &guild_id.to_string(),
            &msg.channel_id.to_string(),
            "channel",
        ).await.unwrap_or(1.0);

        let xp_gain = (settings.xp_per_message as f64 * multiplier) as i64;

        let level_up = data.level_repo.add_xp(
            &guild_id.to_string(),
            &user_id.to_string(),
            xp_gain,
        ).await?;

        if level_up {
            if let Some(ref channel_id) = settings.announce_channel_id {
                if let Ok(ch_id) = channel_id.parse::<u64>() {
                    let channel = poise::serenity_prelude::ChannelId::new(ch_id);
                    let msg_text = settings.level_up_message
                        .replace("{user}", &format!("<@{}>", user_id))
                        .replace("{level}", &data.level_repo.get_level(&guild_id.to_string(), &user_id.to_string()).await?.to_string());
                    
                    if let Err(e) = channel.say(&ctx.http, &msg_text).await {
                        eprintln!("Failed to send level up message: {}", e);
                    }
                }
            }

            Self::check_role_rewards(ctx, data, guild_id, user_id).await?;
        }

        Ok(())
    }

    async fn check_role_rewards(
        ctx: &poise::serenity_prelude::Context,
        data: &crate::BotState,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<(), Error> {
        let level = data.level_repo.get_level(&guild_id.to_string(), &user_id.to_string()).await?;
        
        let rewards = data.level_repo.get_rewards_for_level(&guild_id.to_string(), level).await?;

        for reward in rewards {
            if let Ok(role_id) = reward.role_id.parse::<u64>() {
                let role = poise::serenity_prelude::RoleId::new(role_id);
                let member = guild_id.member(&ctx.http, user_id).await?;
                
                if !member.roles.contains(&role) {
                    member.add_role(&ctx.http, role).await?;
                }
            }
        }

        Ok(())
    }
}
