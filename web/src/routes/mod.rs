mod auth;
mod dashboard;
mod index;
mod settings;
mod settings_pages;

pub use auth::{auth_callback, discord_auth, logout};
pub use dashboard::dashboard;
pub use index::index;
pub use settings::{ping, update_settings};
pub use settings_pages::{bot_settings, moderation_settings, music_settings, ticket_settings, tts_settings};
