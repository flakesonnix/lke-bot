use crate::{session::DiscordUser, AppState};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use shared::UpdateBotSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsRequest {
    pub activity_enabled: bool,
    pub activity_type: String,
    pub activity_name: String,
    pub activity_url: Option<String>,
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

pub async fn ping() -> Response<Body> {
    Json(PingResponse {
        pong: true,
        timestamp: chrono::Utc::now().timestamp_millis(),
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
