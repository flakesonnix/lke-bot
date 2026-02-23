CREATE TABLE custom_commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    response TEXT NOT NULL,
    embed_title TEXT,
    embed_description TEXT,
    embed_color INTEGER,
    embed_image_url TEXT,
    embed_thumbnail_url TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    enabled INTEGER NOT NULL DEFAULT 1,
    cooldown_seconds INTEGER NOT NULL DEFAULT 0,
    require_permissions TEXT,
    UNIQUE(guild_id, name)
);

CREATE INDEX idx_custom_commands_guild ON custom_commands(guild_id);

CREATE TABLE auto_responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    trigger_type TEXT NOT NULL DEFAULT 'contains',
    trigger_pattern TEXT NOT NULL,
    response TEXT NOT NULL,
    response_type TEXT NOT NULL DEFAULT 'text',
    embed_title TEXT,
    embed_description TEXT,
    embed_color INTEGER,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    enabled INTEGER NOT NULL DEFAULT 1,
    case_sensitive INTEGER NOT NULL DEFAULT 0,
    cooldown_seconds INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_auto_responses_guild ON auto_responses(guild_id);

CREATE TABLE command_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    command_type TEXT NOT NULL,
    command_id INTEGER NOT NULL,
    role_id TEXT,
    user_id TEXT,
    allowed INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(command_type, command_id, COALESCE(role_id, ''), COALESCE(user_id, ''))
);

CREATE INDEX idx_command_permissions_command ON command_permissions(command_type, command_id);
