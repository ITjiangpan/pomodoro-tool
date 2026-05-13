pub const CREATE_TABLES: &str = "
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    work_duration INTEGER NOT NULL DEFAULT 25,
    short_break INTEGER NOT NULL DEFAULT 5,
    long_break INTEGER NOT NULL DEFAULT 15,
    long_break_interval INTEGER NOT NULL DEFAULT 4,
    auto_start_break INTEGER NOT NULL DEFAULT 0,
    auto_start_work INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT DEFAULT '',
    completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER,
    duration INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT NOT NULL,
    session_type TEXT NOT NULL DEFAULT 'work',
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

INSERT OR IGNORE INTO settings (id) VALUES (1);
";
