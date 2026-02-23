use crate::{session::DiscordUser, templates::settings, AppState};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn bot_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let settings = state.settings_repo.get().await.ok();
    settings::bot_settings(settings).into_response()
}

pub async fn leveling_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let guild_id = state.config.guild_id.unwrap_or(0).to_string();
    let settings = state.level_repo.get_settings(&guild_id).await.ok();
    let leaderboard = state.level_repo.get_leaderboard(&guild_id, 10).await.unwrap_or_default();
    settings::leveling_settings(settings, leaderboard).into_response()
}

pub async fn ticket_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let settings = state.ticket_settings_repo.get().await.ok();
    settings::ticket_settings(settings).into_response()
}

pub async fn moderation_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let settings = state.moderation_repo.get_settings().await.ok();
    settings::moderation_settings(settings).into_response()
}

pub async fn tts_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let settings = state.tts_repo.get_settings().await.ok();
    settings::tts_settings(settings).into_response()
}

pub async fn music_settings(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_none() {
        return Redirect::to("/").into_response();
    }

    let settings = state.music_repo.get_settings().await.ok();
    settings::music_settings(settings).into_response()
}
