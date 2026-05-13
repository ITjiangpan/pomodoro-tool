mod db;
pub mod timer;
mod task;
mod stats;

use db::Database;
use timer::TimerEngine;
use timer::config::Settings;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("pomodoro.db");
            let db = Database::new(db_path.to_str().unwrap())
                .expect("failed to initialize database");

            // Ensure default settings exist
            {
                let conn = db.conn.lock().unwrap();
                let count: i64 = conn
                    .query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))
                    .unwrap();
                if count == 0 {
                    let default = Settings::default();
                    conn.execute(
                        "INSERT INTO settings (work_duration, short_break, long_break,
                         long_break_interval, auto_start_break, auto_start_work)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![
                            default.work_duration,
                            default.short_break,
                            default.long_break,
                            default.long_break_interval,
                            default.auto_start_break as i32,
                            default.auto_start_work as i32,
                        ],
                    ).unwrap();
                }
            }

            app.manage(db);
            app.manage(TimerEngine::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            task::create_task,
            task::update_task,
            task::delete_task,
            task::list_tasks,
            timer::config::get_settings,
            timer::config::update_settings,
            stats::get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
