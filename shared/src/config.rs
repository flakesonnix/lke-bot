use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub web_base_url: String,
    pub general_channel_id: Option<u64>,
    pub game_name: String,
    pub guild_id: Option<u64>,
    pub session_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            discord_token: env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            discord_client_id: env::var("DISCORD_CLIENT_ID")
                .expect("DISCORD_CLIENT_ID must be set"),
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET")
                .expect("DISCORD_CLIENT_SECRET must be set"),
            web_base_url: env::var("WEB_BASE_URL").expect("WEB_BASE_URL must be set"),
            general_channel_id: env::var("GENERAL_CHANNEL_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            game_name: env::var("GAME_NAME").unwrap_or_else(|_| "with code".to_string()),
            guild_id: env::var("GUILD_ID").ok().and_then(|s| s.parse().ok()),
            session_secret: env::var("SESSION_SECRET")
                .unwrap_or_else(|_| "default-secret-key-please-change-in-production".to_string()),
        }
    }
}
