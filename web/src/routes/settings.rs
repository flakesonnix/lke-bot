use crate::{session::DiscordUser, AppState};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use shared::{UpdateBotSettings, UpdateLevelSettings};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsRequest {
    pub activity_enabled: bool,
    pub activity_type: String,
    pub activity_name: String,
    pub activity_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelingSettingsRequest {
    pub enabled: bool,
    pub xp_per_message: i64,
    pub cooldown_seconds: i64,
    pub announce_channel_id: Option<String>,
    pub level_up_message: String,
}

#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PingResponse {
    pub pong: bool,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct GuildResourcesResponse {
    pub channels: Vec<ChannelInfo>,
    pub roles: Vec<RoleInfo>,
}

#[derive(Debug, Serialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub channel_type: i32,
}

#[derive(Debug, Serialize)]
pub struct RoleInfo {
    pub id: String,
    pub name: String,
    pub color: String,
    pub position: i32,
}

pub async fn ping() -> Response<Body> {
    Json(PingResponse {
        pong: true,
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
    .into_response()
}

pub async fn get_guild_resources(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    let user = match user {
        Some(u) => u,
        None => {
            return Json(GuildResourcesResponse {
                channels: vec![],
                roles: vec![],
            })
            .into_response();
        }
    };

    let guild_id = match state.config.guild_id {
        Some(id) => id.to_string(),
        None => {
            return Json(GuildResourcesResponse {
                channels: vec![],
                roles: vec![],
            })
            .into_response();
        }
    };

    let token: Option<String> = session.get("discord_token").await.ok().flatten();
    let token = match token {
        Some(t) => t,
        None => {
            return Json(GuildResourcesResponse {
                channels: vec![],
                roles: vec![],
            })
            .into_response();
        }
    };

    let channels_url = format!("https://discord.com/api/v10/guilds/{}/channels", guild_id);
    let roles_url = format!("https://discord.com/api/v10/guilds/{}/roles", guild_id);

    let client = &state.http_client;

    let channels_res = client
        .get(&channels_url)
        .bearer_auth(&token)
        .send()
        .await;

    let roles_res = client
        .get(&roles_url)
        .bearer_auth(&token)
        .send()
        .await;

    let channels: Vec<crate::session::DiscordChannel> = match channels_res {
        Ok(res) if res.status().is_success() => {
            res.json().await.unwrap_or_default()
        }
        _ => vec![],
    };

    let roles: Vec<crate::session::DiscordRole> = match roles_res {
        Ok(res) if res.status().is_success() => {
            res.json().await.unwrap_or_default()
        }
        _ => vec![],
    };

    let channel_info: Vec<ChannelInfo> = channels
        .into_iter()
        .filter(|c| c.is_text())
        .map(|c| {
            let icon = c.type_icon().to_string();
            ChannelInfo {
                id: c.id,
                name: c.name,
                icon,
                channel_type: c.channel_type,
            }
        })
        .collect();

    let role_info: Vec<RoleInfo> = roles
        .into_iter()
        .filter(|r| r.name != "@everyone")
        .map(|r| {
            let color = r.color_hex();
            RoleInfo {
                id: r.id,
                name: r.name,
                color,
                position: r.position,
            }
        })
        .collect();

    Json(GuildResourcesResponse {
        channels: channel_info,
        roles: role_info,
    })
    .into_response()
}

pub async fn update_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
    Json(payload): Json<SettingsRequest>,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    let _user = match user {
        Some(u) => u,
        None => {
            return Json(SettingsResponse {
                success: false,
                message: "Not authenticated".to_string(),
            })
            .into_response();
        }
    };

    let guilds: Vec<crate::session::DiscordGuild> = session
        .get("guilds")
        .await
        .ok()
        .flatten()
        .unwrap_or_default();

    let is_admin = guilds.iter().any(|g| g.has_admin() || g.is_owner());

    if !is_admin {
        return Json(SettingsResponse {
            success: false,
            message: "Admin permissions required".to_string(),
        })
        .into_response();
    }

    let valid_types = ["playing", "streaming", "listening", "watching", "competing"];
    if !valid_types.contains(&payload.activity_type.as_str()) {
        return Json(SettingsResponse {
            success: false,
            message: "Invalid activity type".to_string(),
        })
        .into_response();
    }

    if payload.activity_type == "streaming" && payload.activity_url.is_none() {
        return Json(SettingsResponse {
            success: false,
            message: "Streaming requires a URL".to_string(),
        })
        .into_response();
    }

    let settings = UpdateBotSettings {
        activity_enabled: payload.activity_enabled,
        activity_type: payload.activity_type,
        activity_name: payload.activity_name,
        activity_url: payload.activity_url,
    };

    match state.settings_repo.update(settings).await {
        Ok(_) => Json(SettingsResponse {
            success: true,
            message: "Settings updated successfully".to_string(),
        })
        .into_response(),
        Err(e) => Json(SettingsResponse {
            success: false,
            message: format!("Failed to update settings: {}", e),
        })
        .into_response(),
    }
}

pub async fn update_leveling_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
    Json(payload): Json<LevelingSettingsRequest>,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    let _user = match user {
        Some(u) => u,
        None => {
            return Json(SettingsResponse {
                success: false,
                message: "Not authenticated".to_string(),
            })
            .into_response();
        }
    };

    let guilds: Vec<crate::session::DiscordGuild> = session
        .get("guilds")
        .await
        .ok()
        .flatten()
        .unwrap_or_default();

    let is_admin = guilds.iter().any(|g| g.has_admin() || g.is_owner());

    if !is_admin {
        return Json(SettingsResponse {
            success: false,
            message: "Admin permissions required".to_string(),
        })
        .into_response();
    }

    let guild_id = state.config.guild_id.unwrap_or(0).to_string();
    let settings = UpdateLevelSettings {
        xp_per_message: payload.xp_per_message,
        xp_per_minute_voice: 5,
        cooldown_seconds: payload.cooldown_seconds,
        announce_channel_id: payload.announce_channel_id,
        announce_dm: false,
        rank_card_style: "default".to_string(),
        level_up_message: payload.level_up_message,
        enabled: payload.enabled,
    };

    match state.level_repo.update_settings(&guild_id, settings).await {
        Ok(_) => Json(SettingsResponse {
            success: true,
            message: "Leveling settings updated successfully".to_string(),
        })
        .into_response(),
        Err(e) => Json(SettingsResponse {
            success: false,
            message: format!("Failed to update settings: {}", e),
        })
        .into_response(),
    }
}
