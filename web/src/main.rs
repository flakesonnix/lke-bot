mod routes;
mod session;
mod templates;

use axum::routing::{Router, get, post};
use shared::{
    BotSettingsRepository, Config, ModerationRepository, MusicRepository, TicketSettingsRepository,
    TtsRepository, UserRepository, init_db,
};
use std::sync::Arc;
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::SameSite};

pub struct AppState {
    pub config: Config,
    pub user_repo: Arc<UserRepository>,
    pub settings_repo: Arc<BotSettingsRepository>,
    pub ticket_settings_repo: Arc<TicketSettingsRepository>,
    pub moderation_repo: Arc<ModerationRepository>,
    pub tts_repo: Arc<TtsRepository>,
    pub music_repo: Arc<MusicRepository>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let pool = init_db(&config.database_url).await?;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let settings_repo = Arc::new(BotSettingsRepository::new(pool.clone()));
    let ticket_settings_repo = Arc::new(TicketSettingsRepository::new(pool.clone()));
    let moderation_repo = Arc::new(ModerationRepository::new(pool.clone()));
    let tts_repo = Arc::new(TtsRepository::new(pool.clone()));
    let music_repo = Arc::new(MusicRepository::new(pool));

    let http_client = reqwest::Client::new();

    let state = Arc::new(AppState {
        config: config.clone(),
        user_repo,
        settings_repo,
        ticket_settings_repo,
        moderation_repo,
        tts_repo,
        music_repo,
        http_client,
    });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/auth/discord", get(routes::discord_auth))
        .route("/auth/callback", get(routes::auth_callback))
        .route("/dashboard", get(routes::dashboard))
        .route("/settings/bot", get(routes::bot_settings))
        .route("/settings/tickets", get(routes::ticket_settings))
        .route("/settings/moderation", get(routes::moderation_settings))
        .route("/settings/tts", get(routes::tts_settings))
        .route("/settings/music", get(routes::music_settings))
        .route("/api/settings", post(routes::update_settings))
        .route("/api/ping", get(routes::ping))
        .route("/logout", get(routes::logout))
        .layer(session_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    tracing::info!("Web server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
