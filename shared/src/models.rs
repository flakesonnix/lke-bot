use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub discord_id: String,
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub discord_id: String,
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BotSettings {
    pub id: i64,
    pub activity_enabled: bool,
    pub activity_type: String,
    pub activity_name: String,
    pub activity_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBotSettings {
    pub activity_enabled: bool,
    pub activity_type: String,
    pub activity_name: String,
    pub activity_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: i64,
    pub channel_id: String,
    pub guild_id: String,
    pub creator_id: String,
    pub creator_username: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub closed_by: Option<String>,
    pub approved: Option<bool>,
    pub approval_responded_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TicketMessage {
    pub id: i64,
    pub ticket_id: i64,
    pub author_id: String,
    pub author_username: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTicket {
    pub channel_id: String,
    pub guild_id: String,
    pub creator_id: String,
    pub creator_username: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTicketMessage {
    pub ticket_id: i64,
    pub author_id: String,
    pub author_username: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TicketSettings {
    pub id: i64,
    pub enabled: bool,
    pub category_id: Option<String>,
    pub support_role_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub max_open_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketSettings {
    pub enabled: bool,
    pub category_id: Option<String>,
    pub support_role_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub max_open_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModerationSettings {
    pub id: i64,
    pub enabled: bool,
    pub check_bad_words: bool,
    pub check_bad_names: bool,
    pub check_nsfw_avatars: bool,
    pub log_channel_id: Option<String>,
    pub mute_role_id: Option<String>,
    pub warn_threshold: i64,
    pub auto_mute: bool,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModerationSettings {
    pub enabled: bool,
    pub check_bad_words: bool,
    pub check_bad_names: bool,
    pub check_nsfw_avatars: bool,
    pub log_channel_id: Option<String>,
    pub mute_role_id: Option<String>,
    pub warn_threshold: i64,
    pub auto_mute: bool,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModerationWarning {
    pub id: i64,
    pub guild_id: String,
    pub user_id: String,
    pub reason: String,
    pub moderator_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewModerationWarning {
    pub guild_id: String,
    pub user_id: String,
    pub reason: String,
    pub moderator_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomTranslation {
    pub id: i64,
    pub guild_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCustomTranslation {
    pub guild_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TtsSettings {
    pub id: i64,
    pub enabled: bool,
    pub default_voice: String,
    pub default_language: String,
    pub speed: f64,
    pub pitch: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTtsSettings {
    pub enabled: bool,
    pub default_voice: String,
    pub default_language: String,
    pub speed: f64,
    pub pitch: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TtsPermission {
    pub id: i64,
    pub guild_id: String,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    pub can_use_tts: bool,
    pub can_change_voice: bool,
    pub can_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTtsPermission {
    pub guild_id: String,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    pub can_use_tts: bool,
    pub can_change_voice: bool,
    pub can_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildTtsState {
    pub id: i64,
    pub guild_id: String,
    pub enabled: bool,
    pub channel_id: Option<String>,
    pub voice: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGuildTtsState {
    pub guild_id: String,
    pub enabled: bool,
    pub channel_id: Option<String>,
    pub voice: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicSettings {
    pub id: i64,
    pub guest_mode: bool,
    pub stats_visible: bool,
    pub stats_for_guests: bool,
    pub max_queue_size: i64,
    pub default_volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMusicSettings {
    pub guest_mode: bool,
    pub stats_visible: bool,
    pub stats_for_guests: bool,
    pub max_queue_size: i64,
    pub default_volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicStat {
    pub id: i64,
    pub guild_id: String,
    pub track_id: String,
    pub title: String,
    pub artist: Option<String>,
    pub source: String,
    pub played_at: String,
    pub duration_seconds: Option<i64>,
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMusicStat {
    pub guild_id: String,
    pub track_id: String,
    pub title: String,
    pub artist: Option<String>,
    pub source: String,
    pub duration_seconds: Option<i64>,
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MusicPlaytime {
    pub id: i64,
    pub guild_id: String,
    pub date: String,
    pub total_seconds: i64,
    pub track_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TrackStats {
    pub track_id: String,
    pub title: String,
    pub artist: Option<String>,
    pub play_count: i64,
    pub total_duration_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyStats {
    pub date: String,
    pub total_seconds: i64,
    pub track_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicStatsResponse {
    pub top_tracks: Vec<TrackStats>,
    pub daily_stats: Vec<DailyStats>,
    pub total_playtime_seconds: i64,
    pub total_tracks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LevelSettings {
    pub id: i64,
    pub guild_id: String,
    pub xp_per_message: i64,
    pub xp_per_minute_voice: i64,
    pub cooldown_seconds: i64,
    pub announce_channel_id: Option<String>,
    pub announce_dm: bool,
    pub rank_card_style: String,
    pub level_up_message: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLevelSettings {
    pub guild_id: String,
    pub xp_per_message: i64,
    pub xp_per_minute_voice: i64,
    pub cooldown_seconds: i64,
    pub announce_channel_id: Option<String>,
    pub announce_dm: bool,
    pub rank_card_style: String,
    pub level_up_message: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLevelSettings {
    pub xp_per_message: i64,
    pub xp_per_minute_voice: i64,
    pub cooldown_seconds: i64,
    pub announce_channel_id: Option<String>,
    pub announce_dm: bool,
    pub rank_card_style: String,
    pub level_up_message: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLevel {
    pub id: i64,
    pub guild_id: String,
    pub user_id: String,
    pub xp: i64,
    pub level: i64,
    pub total_xp: i64,
    pub last_message_at: Option<String>,
    pub voice_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserLevel {
    pub guild_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LevelReward {
    pub id: i64,
    pub guild_id: String,
    pub level: i64,
    pub role_id: String,
    pub keep_previous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLevelReward {
    pub guild_id: String,
    pub level: i64,
    pub role_id: String,
    pub keep_previous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct XpMultiplier {
    pub id: i64,
    pub guild_id: String,
    pub target_type: String,
    pub target_id: String,
    pub multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewXpMultiplier {
    pub guild_id: String,
    pub target_type: String,
    pub target_id: String,
    pub multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WelcomeSettings {
    pub id: i64,
    pub guild_id: String,
    pub welcome_enabled: bool,
    pub welcome_channel_id: Option<String>,
    pub welcome_message: String,
    pub welcome_dm: bool,
    pub goodbye_enabled: bool,
    pub goodbye_channel_id: Option<String>,
    pub goodbye_message: String,
    pub auto_role_id: Option<String>,
    pub welcome_card_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWelcomeSettings {
    pub guild_id: String,
    pub welcome_enabled: bool,
    pub welcome_channel_id: Option<String>,
    pub welcome_message: String,
    pub welcome_dm: bool,
    pub goodbye_enabled: bool,
    pub goodbye_channel_id: Option<String>,
    pub goodbye_message: String,
    pub auto_role_id: Option<String>,
    pub welcome_card_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWelcomeSettings {
    pub welcome_enabled: bool,
    pub welcome_channel_id: Option<String>,
    pub welcome_message: String,
    pub welcome_dm: bool,
    pub goodbye_enabled: bool,
    pub goodbye_channel_id: Option<String>,
    pub goodbye_message: String,
    pub auto_role_id: Option<String>,
    pub welcome_card_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomCommand {
    pub id: i64,
    pub guild_id: String,
    pub name: String,
    pub description: Option<String>,
    pub response: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub embed_image_url: Option<String>,
    pub embed_thumbnail_url: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub enabled: bool,
    pub cooldown_seconds: i64,
    pub require_permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCustomCommand {
    pub guild_id: String,
    pub name: String,
    pub description: Option<String>,
    pub response: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub embed_image_url: Option<String>,
    pub embed_thumbnail_url: Option<String>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomCommand {
    pub description: Option<String>,
    pub response: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub embed_image_url: Option<String>,
    pub embed_thumbnail_url: Option<String>,
    pub enabled: bool,
    pub cooldown_seconds: i64,
    pub require_permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AutoResponse {
    pub id: i64,
    pub guild_id: String,
    pub trigger_type: String,
    pub trigger_pattern: String,
    pub response: String,
    pub response_type: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub created_by: String,
    pub created_at: String,
    pub enabled: bool,
    pub case_sensitive: bool,
    pub cooldown_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAutoResponse {
    pub guild_id: String,
    pub trigger_type: String,
    pub trigger_pattern: String,
    pub response: String,
    pub response_type: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub created_by: String,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutoResponse {
    pub trigger_type: String,
    pub trigger_pattern: String,
    pub response: String,
    pub response_type: String,
    pub embed_title: Option<String>,
    pub embed_description: Option<String>,
    pub embed_color: Option<i64>,
    pub enabled: bool,
    pub case_sensitive: bool,
    pub cooldown_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommandPermission {
    pub id: i64,
    pub guild_id: String,
    pub command_type: String,
    pub command_id: i64,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    pub allowed: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCommandPermission {
    pub guild_id: String,
    pub command_type: String,
    pub command_id: i64,
    pub role_id: Option<String>,
    pub user_id: Option<String>,
    pub allowed: bool,
}
