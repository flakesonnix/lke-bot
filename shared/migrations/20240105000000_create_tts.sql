CREATE TABLE tts_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    enabled INTEGER NOT NULL DEFAULT 0,
    default_voice TEXT NOT NULL DEFAULT 'en-US',
    default_language TEXT NOT NULL DEFAULT 'en',
    speed REAL NOT NULL DEFAULT 1.0,
    pitch REAL NOT NULL DEFAULT 1.0,
    volume REAL NOT NULL DEFAULT 1.0
);

INSERT INTO tts_settings (id, enabled, default_voice, default_language) VALUES (1, 0, 'en-US', 'en');

CREATE TABLE tts_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    role_id TEXT,
    user_id TEXT,
    can_use_tts INTEGER NOT NULL DEFAULT 0,
    can_change_voice INTEGER NOT NULL DEFAULT 0,
    can_admin INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE guild_tts_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL UNIQUE,
    enabled INTEGER NOT NULL DEFAULT 0,
    channel_id TEXT,
    voice TEXT NOT NULL DEFAULT 'en-US',
    language TEXT NOT NULL DEFAULT 'en'
);
