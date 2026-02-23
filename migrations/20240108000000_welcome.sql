CREATE TABLE welcome_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    guild_id TEXT NOT NULL UNIQUE,
    welcome_enabled INTEGER NOT NULL DEFAULT 0,
    welcome_channel_id TEXT,
    welcome_message TEXT NOT NULL DEFAULT 'Welcome to the server, {user}!',
    welcome_dm INTEGER NOT NULL DEFAULT 0,
    goodbye_enabled INTEGER NOT NULL DEFAULT 0,
    goodbye_channel_id TEXT,
    goodbye_message TEXT NOT NULL DEFAULT 'Goodbye, {user.name}!',
    auto_role_id TEXT,
    welcome_card_enabled INTEGER NOT NULL DEFAULT 0
);
