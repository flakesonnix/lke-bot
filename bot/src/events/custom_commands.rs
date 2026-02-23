use crate::{BotState, Error};
use poise::serenity_prelude::{CreateEmbed, Message, Mentionable};

pub struct CustomCommandHandler;

impl CustomCommandHandler {
    pub async fn handle_message(
        ctx: &poise::serenity_prelude::Context,
        msg: &Message,
        data: &BotState,
    ) -> Result<(), Error> {
        if msg.author.bot {
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let content = msg.content.trim();
        if !content.starts_with('!') {
            return Ok(());
        }

        let command_name = content.trim_start_matches('!').split_whitespace().next().unwrap_or("");
        if command_name.is_empty() {
            return Ok(());
        }

        if let Some(cmd) = data.custom_cmd_repo.get(&guild_id.to_string(), command_name).await? {
            if !cmd.enabled {
                return Ok(());
            }

            let response = Self::substitute_variables(&cmd.response, msg);
            
            if cmd.embed_title.is_some() || cmd.embed_description.is_some() {
                let mut embed = CreateEmbed::new();
                
                if let Some(title) = &cmd.embed_title {
                    embed = embed.title(Self::substitute_variables(title, msg));
                }
                if let Some(desc) = &cmd.embed_description {
                    embed = embed.description(Self::substitute_variables(desc, msg));
                }
                if let Some(color) = cmd.embed_color {
                    embed = embed.color(color as u32);
                }
                if let Some(url) = &cmd.embed_image_url {
                    embed = embed.image(url);
                }
                if let Some(url) = &cmd.embed_thumbnail_url {
                    embed = embed.thumbnail(url);
                }
                
                msg.channel_id.send_message(
                    &ctx.http,
                    poise::serenity_prelude::CreateMessage::new().embed(embed)
                ).await?;
            } else {
                msg.channel_id.say(&ctx.http, &response).await?;
            }
        }

        Ok(())
    }

    fn substitute_variables(text: &str, msg: &Message) -> String {
        // Basic replacements
        let mut result = text.replace("{user}", &msg.author.mention().to_string())
            .replace("{user.name}", &msg.author.name)
            .replace("{user.id}", &msg.author.id.to_string())
            .replace("{channel}", &msg.channel_id.mention().to_string())
            .replace("{server}", &msg.guild_id.map(|g| g.to_string()).unwrap_or_default());
        // Extract arguments after command name for {args}
        let args = msg.content.trim_start_matches('!').splitn(2, ' ').nth(1).unwrap_or("").to_string();
        let mut result = result.replace("{args}", &args);
        // {random:min-max}
        // Find all occurrences using regex and replace with a random number in range
        let random_regex = regex::Regex::new(r"\\{random:(\d+)-(\d+)\\}").unwrap();
        result = random_regex.replace_all(&result, |caps: &regex::Captures| {
            let min: i64 = caps[1].parse().unwrap_or(0);
            let max: i64 = caps[2].parse().unwrap_or(min);
            let range = if max > min { max - min + 1 } else { 1 };
            let val = rand::thread_rng().gen_range(0..range) + min;
            val.to_string()
        }).to_string();
        // {choose:opt1|opt2|opt3}
        let choose_regex = regex::Regex::new(r"\\{choose:([^}]+)\\}").unwrap();
        result = choose_regex.replace_all(&result, |caps: &regex::Captures| {
            let options: Vec<&str> = caps[1].split('|').collect();
            if options.is_empty() {
                String::new()
            } else {
                let idx = rand::thread_rng().gen_range(0..options.len());
                options[idx].to_string()
            }
        }).to_string();
        result
    }
}

pub struct AutoResponseHandler;

impl AutoResponseHandler {
    pub async fn handle_message(
        ctx: &poise::serenity_prelude::Context,
        msg: &Message,
        data: &BotState,
    ) -> Result<(), Error> {
        if msg.author.bot {
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let matches = data.auto_resp_repo.find_matching(&guild_id.to_string(), &msg.content).await?;

        for resp in matches {
            if resp.response_type == "embed" {
                let mut embed = CreateEmbed::new();
                
                if let Some(title) = &resp.embed_title {
                    embed = embed.title(title);
                }
                if let Some(desc) = &resp.embed_description {
                    embed = embed.description(desc);
                }
                if let Some(color) = resp.embed_color {
                    embed = embed.color(color as u32);
                }
                
                msg.channel_id.send_message(
                    &ctx.http,
                    poise::serenity_prelude::CreateMessage::new().embed(embed)
                ).await?;
            } else {
                msg.channel_id.say(&ctx.http, &resp.response).await?;
            }
        }

        Ok(())
    }
}
