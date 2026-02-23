CREATE TABLE level_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    guild_id TEXT NOT NULL UNIQUE,
    xp_per_message INTEGER NOT NULL DEFAULT 15,
    xp_per_minute_voice INTEGER NOT NULL DEFAULT 5,
    cooldown_seconds INTEGER NOT NULL DEFAULT 60,
    announce_channel_id TEXT,
    announce_dm INTEGER NOT NULL DEFAULT 0,
    rank_card_style TEXT NOT NULL DEFAULT 'default',
    level_up_message TEXT NOT NULL DEFAULT '🎉 {user} has reached level {level}!',
    enabled INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE user_levels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    xp INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1,
    total_xp INTEGER NOT NULL DEFAULT 0,
    last_message_at TEXT,
    voice_minutes INTEGER NOT NULL DEFAULT 0,
    UNIQUE(guild_id, user_id)
);

CREATE INDEX idx_user_levels_guild_user ON user_levels(guild_id, user_id);

CREATE TABLE level_rewards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    level INTEGER NOT NULL,
    role_id TEXT NOT NULL,
    keep_previous INTEGER NOT NULL DEFAULT 1,
    UNIQUE(guild_id, level)
);

CREATE TABLE xp_multipliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    multiplier REAL NOT NULL DEFAULT 1.0,
    UNIQUE(guild_id, target_type, target_id)
);
