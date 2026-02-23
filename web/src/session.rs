use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
}

impl DiscordUser {
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png?size=256",
                self.id, hash
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    permissions: String,
}

impl DiscordGuild {
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/icons/{}/{}.png?size=64",
                self.id, hash
            )
        })
    }

    pub fn has_admin(&self) -> bool {
        const ADMINISTRATOR: u64 = 0x8;
        self.permissions
            .parse::<u64>()
            .map(|p| (p & ADMINISTRATOR) != 0)
            .unwrap_or(false)
    }

    pub fn is_owner(&self) -> bool {
        self.owner
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}
