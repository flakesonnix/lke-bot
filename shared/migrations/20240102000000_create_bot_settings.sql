CREATE TABLE bot_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    activity_enabled INTEGER NOT NULL DEFAULT 1,
    activity_type TEXT NOT NULL DEFAULT 'playing',
    activity_name TEXT NOT NULL DEFAULT 'with code',
    activity_url TEXT
);

INSERT INTO bot_settings (id, activity_enabled, activity_type, activity_name) 
VALUES (1, 1, 'playing', 'with code');
