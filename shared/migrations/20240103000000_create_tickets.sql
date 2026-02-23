CREATE TABLE tickets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    channel_id TEXT NOT NULL UNIQUE,
    guild_id TEXT NOT NULL,
    creator_id TEXT NOT NULL,
    creator_username TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TEXT NOT NULL,
    closed_at TEXT,
    closed_by TEXT,
    approved INTEGER,
    approval_responded_at TEXT
);

CREATE TABLE ticket_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ticket_id INTEGER NOT NULL,
    author_id TEXT NOT NULL,
    author_username TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id)
);

CREATE TABLE ticket_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    enabled INTEGER NOT NULL DEFAULT 0,
    category_id TEXT,
    support_role_id TEXT,
    log_channel_id TEXT,
    max_open_days INTEGER NOT NULL DEFAULT 3
);

INSERT INTO ticket_settings (id, enabled, max_open_days) VALUES (1, 0, 3);
