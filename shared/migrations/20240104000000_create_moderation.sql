CREATE TABLE moderation_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    enabled INTEGER NOT NULL DEFAULT 0,
    check_bad_words INTEGER NOT NULL DEFAULT 1,
    check_bad_names INTEGER NOT NULL DEFAULT 1,
    check_nsfw_avatars INTEGER NOT NULL DEFAULT 1,
    log_channel_id TEXT,
    mute_role_id TEXT,
    warn_threshold INTEGER NOT NULL DEFAULT 3,
    auto_mute INTEGER NOT NULL DEFAULT 1,
    language TEXT NOT NULL DEFAULT 'en'
);

INSERT INTO moderation_settings (id, enabled, language) VALUES (1, 0, 'en');

CREATE TABLE moderation_warnings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    moderator_id TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE custom_translations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    UNIQUE(guild_id, key)
);
