use crate::Error;
use poise::serenity_prelude::{Mentionable, Message};
use rustrict::Censor;

pub struct AutoModHandler;

impl AutoModHandler {
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

        let settings = match data.moderation_repo.get_settings().await {
            Ok(s) if s.enabled => s,
            _ => return Ok(()),
        };

        let mut should_delete = false;
        let mut warn_reason = String::new();

        if settings.check_bad_words {
            let censor_result = Censor::from_str(&msg.content);
            if censor_result.is_bad() {
                should_delete = true;
                warn_reason = "Inappropriate language".to_string();
            }
        }

        if settings.check_bad_names {
            let censor_result = Censor::from_str(&msg.author.name);
            if censor_result.is_bad() {
                should_delete = true;
                warn_reason = "Inappropriate username".to_string();
            }
        }

        if Self::is_spam(msg).await {
            should_delete = true;
            warn_reason = "Spam detected".to_string();
        }

        if Self::contains_invite(&msg.content).await {
            should_delete = true;
            warn_reason = "Discord invite link".to_string();
        }

        if should_delete {
            if let Err(e) = msg.delete(&ctx.http).await {
                eprintln!("Failed to delete message: {}", e);
            }

            if let Some(ref log_channel_id) = settings.log_channel_id {
                if let Ok(ch_id) = log_channel_id.parse::<u64>() {
                    let channel = poise::serenity_prelude::ChannelId::new(ch_id);
                    let log_msg = format!(
                        "🗑️ Message by {} deleted in <#{}>\n**Reason:** {}\n**Content:** {}",
                        msg.author.mention(),
                        msg.channel_id,
                        warn_reason,
                        msg.content.chars().take(200).collect::<String>()
                    );
                    let _ = channel.say(&ctx.http, &log_msg).await;
                }
            }

            if let Some(ref mute_role_id) = settings.mute_role_id {
                if settings.auto_mute {
                    if let Ok(r_id) = mute_role_id.parse::<u64>() {
                        let role = poise::serenity_prelude::RoleId::new(r_id);
                        let member = guild_id.member(&ctx.http, msg.author.id).await?;
                        member.add_role(&ctx.http, role).await?;
                    }
                }
            }

            if let Err(e) = msg.author.direct_message(&ctx.http, |m| {
                m.content(&format!(
                    "⚠️ Your message was removed: {}",
                    warn_reason
                ))
            }).await {
                eprintln!("Failed to send DM: {}", e);
            }
        }

        Ok(())
    }

    async fn is_spam(msg: &Message) -> bool {
        let content = &msg.content;
        
        if content.len() > 200 && content.chars().filter(|c| c.is_uppercase()).count() > content.len() / 2 {
            return true;
        }

        if content.len() > 10 {
            let emoji_count = content.chars().filter(|c| {
                let c = *c as u32;
                (0x1F600..=0x1F64F).contains(&c) ||
                (0x1F300..=0x1F5FF).contains(&c) ||
                (0x1F680..=0x1F6FF).contains(&c) ||
                (0x1F1E0..=0x1F1FF).contains(&c) ||
                (0x2600..=0x26FF).contains(&c) ||
                (0x2700..=0x27BF).contains(&c)
            }).count();
            
            if emoji_count > 10 {
                return true;
            }
        }

        false
    }

    async fn contains_invite(content: &str) -> bool {
        content.contains("discord.gg/") || 
        content.contains("discord.com/invite/") ||
        content.contains("discordapp.com/invite/")
    }
}
