pub mod model;

use model::{Task, CreateTaskRequest, UpdateTaskRequest};
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub fn create_task(db: State<'_, Database>, request: CreateTaskRequest) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let desc = request.description.unwrap_or_default();
    conn.execute(
        "INSERT INTO tasks (title, description) VALUES (?1, ?2)",
        rusqlite::params![request.title, desc],
    ).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    let task = conn.query_row(
        "SELECT id, title, description, completed, created_at, 0 as session_count, NULL as last_used_at
         FROM tasks WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            created_at: row.get(4)?,
            session_count: row.get(5)?,
            last_used_at: row.get(6)?,
        }),
    ).map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub fn update_task(db: State<'_, Database>, request: UpdateTaskRequest) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    if let Some(title) = &request.title {
        conn.execute(
            "UPDATE tasks SET title = ?1 WHERE id = ?2",
            rusqlite::params![title, request.id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(completed) = request.completed {
        conn.execute(
            "UPDATE tasks SET completed = ?1 WHERE id = ?2",
            rusqlite::params![completed as i32, request.id],
        ).map_err(|e| e.to_string())?;
    }
    let task = conn.query_row(
        "SELECT t.id, t.title, t.description, t.completed, t.created_at,
                COUNT(p.id) as session_count, t.last_used_at
         FROM tasks t
         LEFT JOIN pomodoro_sessions p ON p.task_id = t.id
         WHERE t.id = ?1
         GROUP BY t.id",
        rusqlite::params![request.id],
        |row| Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            created_at: row.get(4)?,
            session_count: row.get(5)?,
            last_used_at: row.get(6)?,
        }),
    ).map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub fn delete_task(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn list_tasks(db: State<'_, Database>) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT t.id, t.title, t.description, t.completed, t.created_at,
                COUNT(p.id) as session_count, t.last_used_at
         FROM tasks t
         LEFT JOIN pomodoro_sessions p ON p.task_id = t.id
         GROUP BY t.id
         ORDER BY
           CASE WHEN t.last_used_at IS NULL THEN 1 ELSE 0 END,
           t.last_used_at DESC,
           t.completed ASC,
           t.created_at DESC"
    ).map_err(|e| e.to_string())?;

    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            created_at: row.get(4)?,
            session_count: row.get(5)?,
            last_used_at: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(tasks)
}
