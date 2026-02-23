CREATE TABLE music_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    guest_mode INTEGER NOT NULL DEFAULT 0,
    stats_visible INTEGER NOT NULL DEFAULT 1,
    stats_for_guests INTEGER NOT NULL DEFAULT 0,
    max_queue_size INTEGER NOT NULL DEFAULT 100,
    default_volume INTEGER NOT NULL DEFAULT 50
);

INSERT INTO music_settings (id, guest_mode, stats_visible, stats_for_guests) VALUES (1, 0, 1, 0);

CREATE TABLE music_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    track_id TEXT NOT NULL,
    title TEXT NOT NULL,
    artist TEXT,
    source TEXT NOT NULL,
    played_at TEXT NOT NULL,
    duration_seconds INTEGER,
    requested_by TEXT
);

CREATE TABLE music_playtime (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    date TEXT NOT NULL,
    total_seconds INTEGER NOT NULL DEFAULT 0,
    track_count INTEGER NOT NULL DEFAULT 0,
    UNIQUE(guild_id, date)
);

CREATE INDEX idx_music_stats_guild ON music_stats(guild_id);
CREATE INDEX idx_music_stats_track ON music_stats(track_id);
CREATE INDEX idx_music_stats_played_at ON music_stats(played_at);
CREATE INDEX idx_music_playtime_guild_date ON music_playtime(guild_id, date);
