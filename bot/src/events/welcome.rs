use crate::{Context, Error};
use poise::serenity_prelude::{GuildId, Member, User};
use shared::repository::WelcomeRepository;

pub struct WelcomeHandler;

impl WelcomeHandler {
    pub async fn handle_member_join(
        ctx: &poise::serenity_prelude::Context,
        member: &Member,
        data: &crate::BotState,
    ) -> Result<(), Error> {
        let guild_id = member.guild_id;
        let user = &member.user;

        let settings = data.welcome_repo.get_settings(&guild_id.to_string()).await?;

        if !settings.welcome_enabled {
            return Ok(());
        }

        let message = Self::replace_placeholders(&settings.welcome_message, &guild_id, user);

        if settings.welcome_dm {
            if let Err(e) = user.direct_message(&ctx.http, |m| m.content(&message)).await {
                eprintln!("Failed to send welcome DM: {}", e);
            }
        }

        if let Some(ref channel_id) = settings.welcome_channel_id {
            if let Ok(ch_id) = channel_id.parse::<u64>() {
                let channel = poise::serenity_prelude::ChannelId::new(ch_id);
                if let Err(e) = channel.say(&ctx.http, &message).await {
                    eprintln!("Failed to send welcome message: {}", e);
                }
            }
        }

        if let Some(ref role_id) = settings.auto_role_id {
            if let Ok(r_id) = role_id.parse::<u64>() {
                let role = poise::serenity_prelude::RoleId::new(r_id);
                if let Err(e) = guild_id.edit_member(&ctx.http, user.id, |m| m.add_role(role)).await {
                    eprintln!("Failed to add auto role: {}", e);
                }
            }
        }

        Ok(())
    }

    pub async fn handle_member_leave(
        ctx: &poise::serenity_prelude::Context,
        guild_id: GuildId,
        user: &User,
        data: &crate::BotState,
    ) -> Result<(), Error> {
        let settings = data.welcome_repo.get_settings(&guild_id.to_string()).await?;

        if !settings.goodbye_enabled {
            return Ok(());
        }

        let message = Self::replace_placeholders(&settings.goodbye_message, &guild_id, user);

        if let Some(ref channel_id) = settings.goodbye_channel_id {
            if let Ok(ch_id) = channel_id.parse::<u64>() {
                let channel = poise::serenity_prelude::ChannelId::new(ch_id);
                if let Err(e) = channel.say(&ctx.http, &message).await {
                    eprintln!("Failed to send goodbye message: {}", e);
                }
            }
        }

        Ok(())
    }

    fn replace_placeholders(template: &str, guild_id: &GuildId, user: &User) -> String {
        template
            .replace("{user}", &format!("<@{}>", user.id))
            .replace("{user.name}", &user.name)
            .replace("{user.discriminator}", &user.discriminator.map(|d| d.to_string()).unwrap_or_default())
            .replace("{user.id}", &user.id.to_string())
            .replace("{user.avatar}", &user.avatar_url().unwrap_or_default())
            .replace("{server}", &guild_id.to_partial_guild(|g| g.name).unwrap_or_default())
            .replace("{server.id}", &guild_id.to_string())
    }
}
