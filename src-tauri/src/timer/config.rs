use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub work_duration: i64,
    pub short_break: i64,
    pub long_break: i64,
    pub long_break_interval: i64,
    pub auto_start_break: bool,
    pub auto_start_work: bool,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work_duration: 25,
            short_break: 5,
            long_break: 15,
            long_break_interval: 4,
            auto_start_break: true,
            auto_start_work: false,
            theme: "light".to_string(),
        }
    }
}

use crate::db::Database;
use crate::timer::TimerEngine;
use tauri::State;

#[tauri::command]
pub fn get_settings(db: State<'_, Database>) -> Result<Settings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let settings = conn.query_row(
        "SELECT work_duration, short_break, long_break, long_break_interval,
                auto_start_break, auto_start_work, theme FROM settings WHERE id = 1",
        [],
        |row| Ok(Settings {
            work_duration: row.get(0)?,
            short_break: row.get(1)?,
            long_break: row.get(2)?,
            long_break_interval: row.get(3)?,
            auto_start_break: row.get::<_, i32>(4)? != 0,
            auto_start_work: row.get::<_, i32>(5)? != 0,
            theme: row.get::<_, String>(6)?,
        }),
    ).map_err(|e| e.to_string())?;
    Ok(settings)
}

#[tauri::command]
pub fn update_settings(db: State<'_, Database>, settings: Settings) -> Result<Settings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE settings SET work_duration=?1, short_break=?2, long_break=?3,
         long_break_interval=?4, auto_start_break=?5, auto_start_work=?6, theme=?7 WHERE id=1",
        rusqlite::params![
            settings.work_duration,
            settings.short_break,
            settings.long_break,
            settings.long_break_interval,
            settings.auto_start_break as i32,
            settings.auto_start_work as i32,
            settings.theme,
        ],
    ).map_err(|e| e.to_string())?;
    Ok(settings)
}

#[tauri::command]
pub fn clear_all_data(db: State<'_, Database>, engine: State<'_, TimerEngine>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM pomodoro_sessions", []).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks", []).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE settings SET work_duration=25, short_break=5, long_break=15,
         long_break_interval=4, auto_start_break=1, auto_start_work=0, theme='light' WHERE id=1",
        [],
    ).map_err(|e| e.to_string())?;
    engine.reset();
    Ok(())
}
