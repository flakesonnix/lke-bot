use crate::{AppState, session::{DiscordUser, DiscordGuild}};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
};
use base64::Engine;
use sha2::{Digest, Sha256};
use tower_sessions::Session;
use tracing::{info, warn, error};

const DISCORD_AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const DISCORD_TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const DISCORD_USER_URL: &str = "https://discord.com/api/v10/users/@me";
const DISCORD_GUILDS_URL: &str = "https://discord.com/api/v10/users/@me/guilds";

fn generate_random_bytes() -> [u8; 32] {
    use rand::RngExt;
    rand::rng().random()
}

fn generate_state() -> String {
    let bytes = generate_random_bytes();
    hex::encode(bytes)
}

fn generate_pkce_verifier() -> String {
    let bytes = generate_random_bytes();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

pub async fn discord_auth(
    State(state): State<std::sync::Arc<AppState>>,
    session: Session,
) -> Response<Body> {
    let state_param = generate_state();
    let verifier = generate_pkce_verifier();
    let challenge = generate_pkce_challenge(&verifier);

    info!("Starting OAuth flow, state: {}", state_param);

    if let Err(e) = session.insert("oauth_state", state_param.clone()).await {
        error!("Failed to insert oauth_state: {:?}", e);
    }
    if let Err(e) = session.insert("pkce_verifier", verifier).await {
        error!("Failed to insert pkce_verifier: {:?}", e);
    }

    let redirect_uri = format!("{}/auth/callback", state.config.web_base_url);
    info!("Redirect URI: {}", redirect_uri);

    let auth_url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope=identify+guilds&state={}&code_challenge={}&code_challenge_method=S256",
        DISCORD_AUTH_URL,
        state.config.discord_client_id,
        urlencoding::encode(&redirect_uri),
        state_param,
        challenge
    );

    Redirect::to(&auth_url).into_response()
}

pub async fn auth_callback(
    State(app_state): State<std::sync::Arc<AppState>>,
    session: Session,
    axum::extract::Query(params): axum::extract::Query<CallbackParams>,
) -> Response<Body> {
    info!("OAuth callback received, state: {}", params.state);

    let state_param = params.state;
    let code = params.code;

    let stored_state: Option<String> = session.get("oauth_state").await.ok().flatten();
    let verifier: Option<String> = session.get("pkce_verifier").await.ok().flatten();

    info!("Stored state: {:?}, Received state: {}", stored_state, state_param);

    if stored_state.as_ref() != Some(&state_param) {
        warn!("OAuth state mismatch - redirecting to home");
        return Redirect::to("/").into_response();
    }

    let verifier = match verifier {
        Some(v) => v,
        None => {
            warn!("PKCE verifier not found in session");
            return Redirect::to("/").into_response();
        }
    };

    let redirect_uri = format!("{}/auth/callback", app_state.config.web_base_url);

    let params = [
        ("client_id", app_state.config.discord_client_id.as_str()),
        (
            "client_secret",
            app_state.config.discord_client_secret.as_str(),
        ),
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
        ("code_verifier", &verifier),
    ];

    info!("Exchanging code for token...");

    let token_response = app_state
        .http_client
        .post(DISCORD_TOKEN_URL)
        .form(&params)
        .send()
        .await;

    let token_response = match token_response {
        Ok(r) => r,
        Err(e) => {
            error!("Token request failed: {:?}", e);
            return Redirect::to("/").into_response();
        }
    };

    let status = token_response.status();
    let body = match token_response.text().await {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to read response body: {:?}", e);
            return Redirect::to("/").into_response();
        }
    };

    if !status.is_success() {
        error!("Token exchange failed: {} - {}", status, body);
        return Redirect::to("/").into_response();
    }

    info!("Token response body: {}", body);

    let token: crate::session::DiscordTokenResponse = match serde_json::from_str(&body) {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to parse token response: {:?} - body was: {}", e, body);
            return Redirect::to("/").into_response();
        }
    };

    info!("Token received, fetching user...");

    let user_response = app_state
        .http_client
        .get(DISCORD_USER_URL)
        .bearer_auth(&token.access_token)
        .send()
        .await;

    let user_response = match user_response {
        Ok(r) => r,
        Err(e) => {
            error!("User request failed: {:?}", e);
            return Redirect::to("/").into_response();
        }
    };

    let discord_user: DiscordUser = match user_response.json().await {
        Ok(u) => u,
        Err(e) => {
            error!("Failed to parse user response: {:?}", e);
            return Redirect::to("/").into_response();
        }
    };

    info!("Successfully authenticated user: {} ({})", discord_user.username, discord_user.id);

    info!("Fetching user guilds...");
    
    let guilds_response = app_state
        .http_client
        .get(DISCORD_GUILDS_URL)
        .bearer_auth(&token.access_token)
        .send()
        .await;

    let guilds: Vec<DiscordGuild> = match guilds_response {
        Ok(r) => {
            match r.json().await {
                Ok(g) => g,
                Err(e) => {
                    warn!("Failed to parse guilds response: {:?}", e);
                    vec![]
                }
            }
        }
        Err(e) => {
            warn!("Failed to fetch guilds: {:?}", e);
            vec![]
        }
    };

    info!("Found {} guilds", guilds.len());

    let new_user = shared::NewUser {
        discord_id: discord_user.id.clone(),
        username: discord_user.username.clone(),
        discriminator: discord_user.discriminator.clone(),
        avatar_url: discord_user.avatar_url(),
    };

    if let Err(e) = app_state.user_repo.upsert(new_user).await {
        error!("Failed to upsert user: {:?}", e);
    }

    if let Err(e) = session.insert("user", discord_user.clone()).await {
        error!("Failed to insert user into session: {:?}", e);
    } else {
        info!("User inserted into session successfully");
    }
    
    if let Err(e) = session.insert("guilds", guilds).await {
        error!("Failed to insert guilds into session: {:?}", e);
    }

    Redirect::to("/dashboard").into_response()
}

#[derive(serde::Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn logout(session: Session) -> Response<Body> {
    let _ = session.clear().await;
    Redirect::to("/").into_response()
}
