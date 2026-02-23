CREATE TABLE reaction_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    enabled INTEGER NOT NULL DEFAULT 1,
    UNIQUE(guild_id, emoji)
);

CREATE INDEX idx_reaction_roles_guild ON reaction_roles(guild_id);

CREATE TABLE reaction_role_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    title TEXT,
    description TEXT,
    color INTEGER,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(channel_id, message_id)
);

CREATE INDEX idx_reaction_role_messages_guild ON reaction_role_messages(guild_id);

CREATE TABLE reaction_role_message_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message_id INTEGER NOT NULL,
    reaction_role_id INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES reaction_role_messages(id) ON DELETE CASCADE,
    FOREIGN KEY (reaction_role_id) REFERENCES reaction_roles(id) ON DELETE CASCADE,
    UNIQUE(message_id, reaction_role_id)
);
