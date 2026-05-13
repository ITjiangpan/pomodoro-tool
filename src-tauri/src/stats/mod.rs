pub mod model;

use model::{Stats, DailyCount, TaskCount};
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn get_stats(db: State<'_, Database>, range: String) -> Result<Stats, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let total_pomodoros: i64 = conn.query_row(
        "SELECT COUNT(*) FROM pomodoro_sessions WHERE session_type = 'work'",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let total_focus_minutes: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration), 0) FROM pomodoro_sessions WHERE session_type = 'work'",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let completed_tasks: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE completed = 1",
        [],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let (date_filter, date_format) = match range.as_str() {
        "weekly" => ("started_at >= datetime('now', '-7 days')", "%Y-%m-%d"),
        "monthly" => ("started_at >= datetime('now', '-30 days')", "%Y-%m-%d"),
        _ => ("started_at >= datetime('now', '-1 day', 'start of day')", "%Y-%m-%d"),
    };

    let mut stmt = conn.prepare(
        &format!(
            "SELECT strftime('{}', started_at) as day, COUNT(*) as cnt
             FROM pomodoro_sessions
             WHERE session_type = 'work' AND {}
             GROUP BY day ORDER BY day",
            date_format, date_filter
        )
    ).map_err(|e| e.to_string())?;

    let daily_data: Vec<DailyCount> = stmt.query_map([], |row| {
        Ok(DailyCount { date: row.get(0)?, count: row.get(1)? })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    let mut stmt = conn.prepare(
        "SELECT t.title, COUNT(p.id) as cnt
         FROM pomodoro_sessions p
         JOIN tasks t ON t.id = p.task_id
         WHERE p.session_type = 'work'
         GROUP BY t.id
         ORDER BY cnt DESC
         LIMIT 5"
    ).map_err(|e| e.to_string())?;

    let top_tasks: Vec<TaskCount> = stmt.query_map([], |row| {
        Ok(TaskCount { title: row.get(0)?, count: row.get(1)? })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    Ok(Stats { total_pomodoros, total_focus_minutes, completed_tasks, daily_data, top_tasks })
}
