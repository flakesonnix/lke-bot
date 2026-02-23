use crate::{
    BotSettings, CustomTranslation, DailyStats, Error, GuildTtsState, LevelReward, LevelSettings,
    ModerationSettings, ModerationWarning, MusicPlaytime, MusicSettings, MusicStat,
    NewCustomTranslation, NewLevelReward, NewLevelSettings, NewModerationWarning, NewMusicStat,
    NewTicket, NewTicketMessage, NewTtsPermission, NewUser, NewWelcomeSettings, NewXpMultiplier,
    Result, Ticket, TicketMessage, TicketSettings, TrackStats, TtsPermission, TtsSettings,
    UpdateBotSettings, UpdateGuildTtsState, UpdateLevelSettings, UpdateModerationSettings,
    UpdateMusicSettings, UpdateTicketSettings, UpdateTtsSettings, UpdateUser, UpdateWelcomeSettings,
    User, UserLevel, WelcomeSettings, XpMultiplier,
};
use sqlx::SqlitePool;

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_discord_id(&self, discord_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE discord_id = ?")
            .bind(discord_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create(&self, user: NewUser) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();

        let result = sqlx::query(
            r#"
            INSERT INTO users (discord_id, username, discriminator, avatar_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&user.discord_id)
        .bind(&user.username)
        .bind(&user.discriminator)
        .bind(&user.avatar_url)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.find_by_id(result.last_insert_rowid())
            .await?
            .ok_or_else(|| Error::NotFound("User not found after insert".into()))
    }

    pub async fn update(&self, discord_id: &str, user: UpdateUser) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE users 
            SET username = ?, discriminator = ?, avatar_url = ?, updated_at = ?
            WHERE discord_id = ?
            "#,
        )
        .bind(&user.username)
        .bind(&user.discriminator)
        .bind(&user.avatar_url)
        .bind(&now)
        .bind(discord_id)
        .execute(&self.pool)
        .await?;

        self.find_by_discord_id(discord_id)
            .await?
            .ok_or_else(|| Error::NotFound("User not found after update".into()))
    }

    pub async fn upsert(&self, user: NewUser) -> Result<User> {
        if self.find_by_discord_id(&user.discord_id).await?.is_some() {
            let update = UpdateUser {
                username: user.username,
                discriminator: user.discriminator,
                avatar_url: user.avatar_url,
            };
            self.update(&user.discord_id, update).await
        } else {
            self.create(user).await
        }
    }

    pub async fn list_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn count(&self) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }
}

pub struct BotSettingsRepository {
    pool: SqlitePool,
}

impl BotSettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self) -> Result<BotSettings> {
        let settings = sqlx::query_as::<_, BotSettings>("SELECT * FROM bot_settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        settings.ok_or_else(|| Error::NotFound("Bot settings not found".into()))
    }

    pub async fn update(&self, settings: UpdateBotSettings) -> Result<BotSettings> {
        sqlx::query(
            r#"
            UPDATE bot_settings 
            SET activity_enabled = ?, activity_type = ?, activity_name = ?, activity_url = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.activity_enabled)
        .bind(&settings.activity_type)
        .bind(&settings.activity_name)
        .bind(&settings.activity_url)
        .execute(&self.pool)
        .await?;

        self.get().await
    }
}

pub struct TicketSettingsRepository {
    pool: SqlitePool,
}

impl TicketSettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self) -> Result<TicketSettings> {
        let settings = sqlx::query_as::<_, TicketSettings>("SELECT * FROM ticket_settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        settings.ok_or_else(|| Error::NotFound("Ticket settings not found".into()))
    }

    pub async fn update(&self, settings: UpdateTicketSettings) -> Result<TicketSettings> {
        sqlx::query(
            r#"
            UPDATE ticket_settings 
            SET enabled = ?, category_id = ?, support_role_id = ?, log_channel_id = ?, max_open_days = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.enabled)
        .bind(&settings.category_id)
        .bind(&settings.support_role_id)
        .bind(&settings.log_channel_id)
        .bind(settings.max_open_days)
        .execute(&self.pool)
        .await?;

        self.get().await
    }
}

pub struct TicketRepository {
    pool: SqlitePool,
}

impl TicketRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, ticket: NewTicket) -> Result<Ticket> {
        let now = chrono::Utc::now().to_rfc3339();

        let result = sqlx::query(
            r#"
            INSERT INTO tickets (channel_id, guild_id, creator_id, creator_username, title, status, created_at)
            VALUES (?, ?, ?, ?, ?, 'open', ?)
            "#
        )
        .bind(&ticket.channel_id)
        .bind(&ticket.guild_id)
        .bind(&ticket.creator_id)
        .bind(&ticket.creator_username)
        .bind(&ticket.title)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.find_by_id(result.last_insert_rowid())
            .await?
            .ok_or_else(|| Error::NotFound("Ticket not found after insert".into()))
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Ticket>> {
        let ticket = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(ticket)
    }

    pub async fn find_by_channel(&self, channel_id: &str) -> Result<Option<Ticket>> {
        let ticket = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets WHERE channel_id = ?")
            .bind(channel_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(ticket)
    }

    pub async fn find_open_by_creator(&self, creator_id: &str, guild_id: &str) -> Result<Option<Ticket>> {
        let ticket = sqlx::query_as::<_, Ticket>(
            "SELECT * FROM tickets WHERE creator_id = ? AND guild_id = ? AND status = 'open'"
        )
        .bind(creator_id)
        .bind(guild_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(ticket)
    }

    pub async fn close(&self, id: i64, closed_by: &str) -> Result<Ticket> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE tickets SET status = 'closed', closed_at = ?, closed_by = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(closed_by)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Ticket not found after close".into()))
    }

    pub async fn approve(&self, id: i64, approved: bool) -> Result<Ticket> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE tickets SET approved = ?, approval_responded_at = ? WHERE id = ?"
        )
        .bind(approved)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound("Ticket not found after approval".into()))
    }

    pub async fn add_message(&self, message: NewTicketMessage) -> Result<TicketMessage> {
        let now = chrono::Utc::now().to_rfc3339();

        let result = sqlx::query(
            "INSERT INTO ticket_messages (ticket_id, author_id, author_username, content, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(message.ticket_id)
        .bind(&message.author_id)
        .bind(&message.author_username)
        .bind(&message.content)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, TicketMessage>("SELECT * FROM ticket_messages WHERE id = ?")
            .bind(result.last_insert_rowid())
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| Error::NotFound("Message not found after insert".into()))
    }

    pub async fn get_messages(&self, ticket_id: i64) -> Result<Vec<TicketMessage>> {
        let messages = sqlx::query_as::<_, TicketMessage>(
            "SELECT * FROM ticket_messages WHERE ticket_id = ? ORDER BY created_at ASC"
        )
        .bind(ticket_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }

    pub async fn list_by_guild(&self, guild_id: &str, limit: i64) -> Result<Vec<Ticket>> {
        let tickets = sqlx::query_as::<_, Ticket>(
            "SELECT * FROM tickets WHERE guild_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(guild_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }
}

pub struct ModerationRepository {
    pool: SqlitePool,
}

impl ModerationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_settings(&self) -> Result<ModerationSettings> {
        let settings = sqlx::query_as::<_, ModerationSettings>(
            "SELECT * FROM moderation_settings WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        settings.ok_or_else(|| Error::NotFound("Moderation settings not found".into()))
    }

    pub async fn update_settings(&self, settings: UpdateModerationSettings) -> Result<ModerationSettings> {
        sqlx::query(
            r#"
            UPDATE moderation_settings 
            SET enabled = ?, check_bad_words = ?, check_bad_names = ?, check_nsfw_avatars = ?,
                log_channel_id = ?, mute_role_id = ?, warn_threshold = ?, auto_mute = ?, language = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.enabled)
        .bind(settings.check_bad_words)
        .bind(settings.check_bad_names)
        .bind(settings.check_nsfw_avatars)
        .bind(&settings.log_channel_id)
        .bind(&settings.mute_role_id)
        .bind(settings.warn_threshold)
        .bind(settings.auto_mute)
        .bind(&settings.language)
        .execute(&self.pool)
        .await?;

        self.get_settings().await
    }

    pub async fn add_warning(&self, warning: NewModerationWarning) -> Result<ModerationWarning> {
        let now = chrono::Utc::now().to_rfc3339();

        let result = sqlx::query(
            "INSERT INTO moderation_warnings (guild_id, user_id, reason, moderator_id, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&warning.guild_id)
        .bind(&warning.user_id)
        .bind(&warning.reason)
        .bind(&warning.moderator_id)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, ModerationWarning>("SELECT * FROM moderation_warnings WHERE id = ?")
            .bind(result.last_insert_rowid())
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| Error::NotFound("Warning not found after insert".into()))
    }

    pub async fn get_warnings(&self, guild_id: &str, user_id: &str) -> Result<Vec<ModerationWarning>> {
        let warnings = sqlx::query_as::<_, ModerationWarning>(
            "SELECT * FROM moderation_warnings WHERE guild_id = ? AND user_id = ? ORDER BY created_at DESC"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(warnings)
    }

    pub async fn get_warning_count(&self, guild_id: &str, user_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM moderation_warnings WHERE guild_id = ? AND user_id = ?"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    pub async fn get_translation(&self, guild_id: &str, key: &str) -> Result<Option<CustomTranslation>> {
        let translation = sqlx::query_as::<_, CustomTranslation>(
            "SELECT * FROM custom_translations WHERE guild_id = ? AND key = ?"
        )
        .bind(guild_id)
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(translation)
    }

    pub async fn set_translation(&self, translation: NewCustomTranslation) -> Result<CustomTranslation> {
        sqlx::query(
            "INSERT OR REPLACE INTO custom_translations (guild_id, key, value) VALUES (?, ?, ?)"
        )
        .bind(&translation.guild_id)
        .bind(&translation.key)
        .bind(&translation.value)
        .execute(&self.pool)
        .await?;

        self.get_translation(&translation.guild_id, &translation.key)
            .await?
            .ok_or_else(|| Error::NotFound("Translation not found after insert".into()))
    }

    pub async fn get_all_translations(&self, guild_id: &str) -> Result<Vec<CustomTranslation>> {
        let translations = sqlx::query_as::<_, CustomTranslation>(
            "SELECT * FROM custom_translations WHERE guild_id = ?"
        )
        .bind(guild_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(translations)
    }

    pub async fn delete_translation(&self, guild_id: &str, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM custom_translations WHERE guild_id = ? AND key = ?")
            .bind(guild_id)
            .bind(key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

pub struct TtsRepository {
    pool: SqlitePool,
}

impl TtsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_settings(&self) -> Result<TtsSettings> {
        let settings = sqlx::query_as::<_, TtsSettings>("SELECT * FROM tts_settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        settings.ok_or_else(|| Error::NotFound("TTS settings not found".into()))
    }

    pub async fn update_settings(&self, settings: UpdateTtsSettings) -> Result<TtsSettings> {
        sqlx::query(
            r#"
            UPDATE tts_settings 
            SET enabled = ?, default_voice = ?, default_language = ?, speed = ?, pitch = ?, volume = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.enabled)
        .bind(&settings.default_voice)
        .bind(&settings.default_language)
        .bind(settings.speed)
        .bind(settings.pitch)
        .bind(settings.volume)
        .execute(&self.pool)
        .await?;

        self.get_settings().await
    }

    pub async fn get_guild_state(&self, guild_id: &str) -> Result<Option<GuildTtsState>> {
        let state = sqlx::query_as::<_, GuildTtsState>(
            "SELECT * FROM guild_tts_state WHERE guild_id = ?",
        )
        .bind(guild_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(state)
    }

    pub async fn set_guild_state(&self, state: UpdateGuildTtsState) -> Result<GuildTtsState> {
        sqlx::query(
            r#"
            INSERT INTO guild_tts_state (guild_id, enabled, channel_id, voice, language)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(guild_id) DO UPDATE SET enabled = ?, channel_id = ?, voice = ?, language = ?
            "#,
        )
        .bind(&state.guild_id)
        .bind(state.enabled)
        .bind(&state.channel_id)
        .bind(&state.voice)
        .bind(&state.language)
        .bind(state.enabled)
        .bind(&state.channel_id)
        .bind(&state.voice)
        .bind(&state.language)
        .execute(&self.pool)
        .await?;

        self.get_guild_state(&state.guild_id)
            .await?
            .ok_or_else(|| Error::NotFound("Guild TTS state not found".into()))
    }

    pub async fn add_permission(&self, perm: NewTtsPermission) -> Result<TtsPermission> {
        let result = sqlx::query(
            r#"
            INSERT INTO tts_permissions (guild_id, role_id, user_id, can_use_tts, can_change_voice, can_admin)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&perm.guild_id)
        .bind(&perm.role_id)
        .bind(&perm.user_id)
        .bind(perm.can_use_tts)
        .bind(perm.can_change_voice)
        .bind(perm.can_admin)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, TtsPermission>("SELECT * FROM tts_permissions WHERE id = ?")
            .bind(result.last_insert_rowid())
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| Error::NotFound("TTS permission not found after insert".into()))
    }

    pub async fn get_permissions(&self, guild_id: &str) -> Result<Vec<TtsPermission>> {
        let permissions = sqlx::query_as::<_, TtsPermission>(
            "SELECT * FROM tts_permissions WHERE guild_id = ?",
        )
        .bind(guild_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(permissions)
    }

    pub async fn delete_permission(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM tts_permissions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn check_permission(&self, guild_id: &str, user_id: &str, role_ids: &[String]) -> Result<Option<TtsPermission>> {
        for role_id in role_ids {
            let perm = sqlx::query_as::<_, TtsPermission>(
                "SELECT * FROM tts_permissions WHERE guild_id = ? AND role_id = ? AND can_use_tts = 1"
            )
            .bind(guild_id)
            .bind(role_id)
            .fetch_optional(&self.pool)
            .await?;

            if perm.is_some() {
                return Ok(perm);
            }
        }

        let perm = sqlx::query_as::<_, TtsPermission>(
            "SELECT * FROM tts_permissions WHERE guild_id = ? AND user_id = ? AND can_use_tts = 1"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(perm)
    }
}

pub struct MusicRepository {
    pool: SqlitePool,
}

impl MusicRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_settings(&self) -> Result<MusicSettings> {
        let settings = sqlx::query_as::<_, MusicSettings>("SELECT * FROM music_settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        settings.ok_or_else(|| Error::NotFound("Music settings not found".into()))
    }

    pub async fn update_settings(&self, settings: UpdateMusicSettings) -> Result<MusicSettings> {
        sqlx::query(
            r#"
            UPDATE music_settings 
            SET guest_mode = ?, stats_visible = ?, stats_for_guests = ?, max_queue_size = ?, default_volume = ?
            WHERE id = 1
            "#,
        )
        .bind(settings.guest_mode)
        .bind(settings.stats_visible)
        .bind(settings.stats_for_guests)
        .bind(settings.max_queue_size)
        .bind(settings.default_volume)
        .execute(&self.pool)
        .await?;

        self.get_settings().await
    }

    pub async fn record_play(&self, stat: NewMusicStat) -> Result<MusicStat> {
        let now = chrono::Utc::now().to_rfc3339();
        let today = now.split('T').next().unwrap_or(&now);

        let result = sqlx::query(
            "INSERT INTO music_stats (guild_id, track_id, title, artist, source, played_at, duration_seconds, requested_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&stat.guild_id)
        .bind(&stat.track_id)
        .bind(&stat.title)
        .bind(&stat.artist)
        .bind(&stat.source)
        .bind(&now)
        .bind(stat.duration_seconds)
        .bind(&stat.requested_by)
        .execute(&self.pool)
        .await?;

        if let Some(duration) = stat.duration_seconds {
            sqlx::query(
                r#"
                INSERT INTO music_playtime (guild_id, date, total_seconds, track_count)
                VALUES (?, ?, ?, 1)
                ON CONFLICT(guild_id, date) DO UPDATE SET 
                    total_seconds = total_seconds + ?,
                    track_count = track_count + 1
                "#,
            )
            .bind(&stat.guild_id)
            .bind(today)
            .bind(duration)
            .bind(duration)
            .execute(&self.pool)
            .await?;
        }

        sqlx::query_as::<_, MusicStat>("SELECT * FROM music_stats WHERE id = ?")
            .bind(result.last_insert_rowid())
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| Error::NotFound("Music stat not found after insert".into()))
    }

    pub async fn get_top_tracks(&self, guild_id: &str, start_date: &str, end_date: &str, limit: i64) -> Result<Vec<TrackStats>> {
        let stats = sqlx::query_as::<_, TrackStats>(
            r#"
            SELECT track_id, MAX(title) as title, MAX(artist) as artist, COUNT(*) as play_count, 
                   COALESCE(SUM(duration_seconds), 0) as total_duration_seconds
            FROM music_stats 
            WHERE guild_id = ? AND played_at >= ? AND played_at <= ?
            GROUP BY track_id
            ORDER BY play_count DESC
            LIMIT ?
            "#,
        )
        .bind(guild_id)
        .bind(start_date)
        .bind(end_date)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    pub async fn get_daily_stats(&self, guild_id: &str, start_date: &str, end_date: &str) -> Result<Vec<DailyStats>> {
        let stats = sqlx::query_as::<_, DailyStats>(
            r#"
            SELECT date, total_seconds, track_count
            FROM music_playtime
            WHERE guild_id = ? AND date >= ? AND date <= ?
            ORDER BY date ASC
            "#,
        )
        .bind(guild_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    pub async fn get_total_stats(&self, guild_id: &str, start_date: &str, end_date: &str) -> Result<(i64, i64)> {
        let result: (i64, i64) = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(total_seconds), 0), COALESCE(SUM(track_count), 0)
            FROM music_playtime
            WHERE guild_id = ? AND date >= ? AND date <= ?
            "#,
        )
        .bind(guild_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}

pub struct LevelRepository {
    pool: SqlitePool,
}

impl LevelRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_settings(&self, guild_id: &str) -> Result<LevelSettings> {
        let settings = sqlx::query_as::<_, LevelSettings>(
            "SELECT * FROM level_settings WHERE guild_id = ?"
        )
        .bind(guild_id)
        .fetch_optional(&self.pool)
        .await?;

        settings.ok_or_else(|| Error::NotFound("Level settings not found".into()))
    }

    pub async fn create_settings(&self, settings: NewLevelSettings) -> Result<LevelSettings> {
        sqlx::query(
            r#"
            INSERT INTO level_settings (guild_id, xp_per_message, xp_per_minute_voice, cooldown_seconds,
                announce_channel_id, announce_dm, rank_card_style, level_up_message, enabled)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&settings.guild_id)
        .bind(settings.xp_per_message)
        .bind(settings.xp_per_minute_voice)
        .bind(settings.cooldown_seconds)
        .bind(&settings.announce_channel_id)
        .bind(settings.announce_dm)
        .bind(&settings.rank_card_style)
        .bind(&settings.level_up_message)
        .bind(settings.enabled)
        .execute(&self.pool)
        .await?;

        self.get_settings(&settings.guild_id).await
    }

    pub async fn update_settings(&self, guild_id: &str, settings: UpdateLevelSettings) -> Result<LevelSettings> {
        sqlx::query(
            r#"
            UPDATE level_settings 
            SET xp_per_message = ?, xp_per_minute_voice = ?, cooldown_seconds = ?,
                announce_channel_id = ?, announce_dm = ?, rank_card_style = ?,
                level_up_message = ?, enabled = ?
            WHERE guild_id = ?
            "#,
        )
        .bind(settings.xp_per_message)
        .bind(settings.xp_per_minute_voice)
        .bind(settings.cooldown_seconds)
        .bind(&settings.announce_channel_id)
        .bind(settings.announce_dm)
        .bind(&settings.rank_card_style)
        .bind(&settings.level_up_message)
        .bind(settings.enabled)
        .bind(guild_id)
        .execute(&self.pool)
        .await?;

        self.get_settings(guild_id).await
    }

    pub async fn get_user_level(&self, guild_id: &str, user_id: &str) -> Result<UserLevel> {
        let level = sqlx::query_as::<_, UserLevel>(
            "SELECT * FROM user_levels WHERE guild_id = ? AND user_id = ?"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        level.ok_or_else(|| Error::NotFound("User level not found".into()))
    }

    pub async fn get_or_create_user_level(&self, guild_id: &str, user_id: &str) -> Result<UserLevel> {
        if let Some(level) = sqlx::query_as::<_, UserLevel>(
            "SELECT * FROM user_levels WHERE guild_id = ? AND user_id = ?"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        {
            return Ok(level);
        }

        sqlx::query(
            "INSERT INTO user_levels (guild_id, user_id, xp, level, total_xp) VALUES (?, ?, 0, 1, 0)"
        )
        .bind(guild_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_user_level(guild_id, user_id).await
    }

    pub async fn check_cooldown(&self, guild_id: &str, user_id: &str, cooldown_seconds: i64) -> Result<bool> {
        let last_message: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT last_message_at FROM user_levels WHERE guild_id = ? AND user_id = ?"
        )
        .bind(guild_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match last_message {
            Some((Some(last),)) => {
                if let Ok(last_time) = chrono::DateTime::parse_from_rfc3339(&last) {
                    let last_utc = last_time.with_timezone(&chrono::Utc);
                    let now = chrono::Utc::now();
                    let elapsed = (now - last_utc).num_seconds();
                    return Ok(elapsed >= cooldown_seconds);
                }
                Ok(true)
            }
            _ => Ok(true),
        }
    }

    pub async fn add_xp(&self, guild_id: &str, user_id: &str, amount: i64) -> Result<bool> {
        let mut user = self.get_or_create_user_level(guild_id, user_id).await?;
        let old_level = user.level;

        user.xp += amount;
        user.total_xp += amount;

        let new_level = Self::calculate_level(user.total_xp);
        user.level = new_level;

        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE user_levels 
            SET xp = ?, level = ?, total_xp = ?, last_message_at = ?
            WHERE guild_id = ? AND user_id = ?
            "#,
        )
        .bind(user.xp)
        .bind(user.level)
        .bind(user.total_xp)
        .bind(&now)
        .bind(guild_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(new_level > old_level)
    }

    pub async fn get_level(&self, guild_id: &str, user_id: &str) -> Result<i64> {
        let user = self.get_or_create_user_level(guild_id, user_id).await?;
        Ok(user.level)
    }

    fn calculate_level(total_xp: i64) -> i64 {
        let mut level = 1i64;
        let mut xp_needed = 100i64;
        let mut accumulated = 0i64;

        while accumulated + xp_needed <= total_xp {
            accumulated += xp_needed;
            level += 1;
            xp_needed = (xp_needed as f64 * 1.5) as i64;
        }

        level
    }

    pub async fn get_multiplier(&self, guild_id: &str, target_id: &str, target_type: &str) -> Result<f64> {
        let mult: Option<(f64,)> = sqlx::query_as(
            "SELECT multiplier FROM xp_multipliers WHERE guild_id = ? AND target_id = ? AND target_type = ?"
        )
        .bind(guild_id)
        .bind(target_id)
        .bind(target_type)
        .fetch_optional(&self.pool)
        .await?;

        Ok(mult.map(|m| m.0).unwrap_or(1.0))
    }

    pub async fn add_reward(&self, reward: NewLevelReward) -> Result<LevelReward> {
        sqlx::query(
            "INSERT INTO level_rewards (guild_id, level, role_id, keep_previous) VALUES (?, ?, ?, ?)"
        )
        .bind(&reward.guild_id)
        .bind(reward.level)
        .bind(&reward.role_id)
        .bind(reward.keep_previous)
        .execute(&self.pool)
        .await?;

        let reward = sqlx::query_as::<_, LevelReward>(
            "SELECT * FROM level_rewards WHERE guild_id = ? AND level = ?"
        )
        .bind(&reward.guild_id)
        .bind(reward.level)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound("Reward not found after insert".into()))?;

        Ok(reward)
    }

    pub async fn get_rewards_for_level(&self, guild_id: &str, level: i64) -> Result<Vec<LevelReward>> {
        let rewards = sqlx::query_as::<_, LevelReward>(
            "SELECT * FROM level_rewards WHERE guild_id = ? AND level <= ? ORDER BY level ASC"
        )
        .bind(guild_id)
        .bind(level)
        .fetch_all(&self.pool)
        .await?;

        Ok(rewards)
    }

    pub async fn get_leaderboard(&self, guild_id: &str, limit: i64) -> Result<Vec<UserLevel>> {
        let leaderboard = sqlx::query_as::<_, UserLevel>(
            "SELECT * FROM user_levels WHERE guild_id = ? ORDER BY total_xp DESC LIMIT ?"
        )
        .bind(guild_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(leaderboard)
    }

    pub async fn set_xp(&self, guild_id: &str, user_id: &str, amount: i64) -> Result<UserLevel> {
        let level = Self::calculate_level(amount);

        sqlx::query(
            r#"
            UPDATE user_levels SET xp = ?, total_xp = ?, level = ? WHERE guild_id = ? AND user_id = ?
            "#,
        )
        .bind(amount)
        .bind(amount)
        .bind(level)
        .bind(guild_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_user_level(guild_id, user_id).await
    }

    pub async fn add_multiplier(&self, mult: NewXpMultiplier) -> Result<XpMultiplier> {
        sqlx::query(
            r#"
            INSERT INTO xp_multipliers (guild_id, target_type, target_id, multiplier)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(guild_id, target_type, target_id) DO UPDATE SET multiplier = ?
            "#,
        )
        .bind(&mult.guild_id)
        .bind(&mult.target_type)
        .bind(&mult.target_id)
        .bind(mult.multiplier)
        .bind(mult.multiplier)
        .execute(&self.pool)
        .await?;

        let mult = sqlx::query_as::<_, XpMultiplier>(
            "SELECT * FROM xp_multipliers WHERE guild_id = ? AND target_type = ? AND target_id = ?"
        )
        .bind(&mult.guild_id)
        .bind(&mult.target_type)
        .bind(&mult.target_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound("Multiplier not found".into()))?;

        Ok(mult)
    }
}

pub struct WelcomeRepository {
    pool: SqlitePool,
}

impl WelcomeRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_settings(&self, guild_id: &str) -> Result<WelcomeSettings> {
        let settings = sqlx::query_as::<_, WelcomeSettings>(
            "SELECT * FROM welcome_settings WHERE guild_id = ?"
        )
        .bind(guild_id)
        .fetch_optional(&self.pool)
        .await?;

        settings.ok_or_else(|| Error::NotFound("Welcome settings not found".into()))
    }

    pub async fn create_settings(&self, settings: NewWelcomeSettings) -> Result<WelcomeSettings> {
        sqlx::query(
            r#"
            INSERT INTO welcome_settings (guild_id, welcome_enabled, welcome_channel_id, welcome_message,
                welcome_dm, goodbye_enabled, goodbye_channel_id, goodbye_message, auto_role_id, welcome_card_enabled)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&settings.guild_id)
        .bind(settings.welcome_enabled)
        .bind(&settings.welcome_channel_id)
        .bind(&settings.welcome_message)
        .bind(settings.welcome_dm)
        .bind(settings.goodbye_enabled)
        .bind(&settings.goodbye_channel_id)
        .bind(&settings.goodbye_message)
        .bind(&settings.auto_role_id)
        .bind(settings.welcome_card_enabled)
        .execute(&self.pool)
        .await?;

        self.get_settings(&settings.guild_id).await
    }

    pub async fn update_settings(&self, guild_id: &str, settings: UpdateWelcomeSettings) -> Result<WelcomeSettings> {
        sqlx::query(
            r#"
            UPDATE welcome_settings 
            SET welcome_enabled = ?, welcome_channel_id = ?, welcome_message = ?,
                welcome_dm = ?, goodbye_enabled = ?, goodbye_channel_id = ?,
                goodbye_message = ?, auto_role_id = ?, welcome_card_enabled = ?
            WHERE guild_id = ?
            "#,
        )
        .bind(settings.welcome_enabled)
        .bind(&settings.welcome_channel_id)
        .bind(&settings.welcome_message)
        .bind(settings.welcome_dm)
        .bind(settings.goodbye_enabled)
        .bind(&settings.goodbye_channel_id)
        .bind(&settings.goodbye_message)
        .bind(&settings.auto_role_id)
        .bind(settings.welcome_card_enabled)
        .bind(guild_id)
        .execute(&self.pool)
        .await?;

        self.get_settings(guild_id).await
    }
}
