use crate::{session::{DiscordGuild, DiscordUser}, templates, AppState};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn dashboard(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let user: Option<DiscordUser> = session.get("user").await.ok().flatten();

    let user = match user {
        Some(u) => u,
        None => return Redirect::to("/").into_response(),
    };

    let guilds: Vec<DiscordGuild> = session
        .get("guilds")
        .await
        .ok()
        .flatten()
        .unwrap_or_default();

    let db_user = state
        .user_repo
        .find_by_discord_id(&user.id)
        .await
        .ok()
        .flatten();

    let user_count = state.user_repo.count().await.unwrap_or(0);

    templates::dashboard(user, db_user, user_count, guilds).into_response()
}
