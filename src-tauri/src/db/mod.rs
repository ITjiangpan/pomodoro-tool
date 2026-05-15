pub mod migrations;

use rusqlite::Connection;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(migrations::CREATE_TABLES)?;
        // Migration: add last_used_at column (ignore if already exists)
        let _ = conn.execute("ALTER TABLE tasks ADD COLUMN last_used_at TEXT", []);
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
