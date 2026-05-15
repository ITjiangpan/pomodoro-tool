pub mod config;
pub mod state;

use state::{TimerPhase, TimerState};
use config::Settings;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TimerEngine {
    pub state: Arc<Mutex<TimerState>>,
}

impl TimerEngine {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(TimerState::new())),
        }
    }

    pub fn start_work(&self, settings: &Settings, task_id: Option<i64>) -> TimerState {
        let mut s = self.state.lock().unwrap();
        *s = TimerState {
            phase: TimerPhase::Working,
            elapsed_secs: 0,
            total_secs: (settings.work_duration as u64) * 60,
            current_session_id: None,
            task_id,
            completed_pomodoros: s.completed_pomodoros,
        };
        s.clone()
    }

    pub fn pause(&self) -> TimerState {
        let mut s = self.state.lock().unwrap();
        if matches!(s.phase, TimerPhase::Working | TimerPhase::ShortBreak | TimerPhase::LongBreak) {
            s.phase = TimerPhase::Paused;
        }
        s.clone()
    }

    pub fn resume(&self) -> TimerState {
        let mut s = self.state.lock().unwrap();
        if s.phase == TimerPhase::Paused {
            if s.task_id.is_some() {
                s.phase = TimerPhase::Working;
            } else {
                s.phase = TimerPhase::ShortBreak;
            }
        }
        s.clone()
    }

    pub fn tick(&self) -> (bool, TimerState) {
        let mut s = self.state.lock().unwrap();
        let completed = s.tick();
        (completed, s.clone())
    }

    pub fn reset(&self) -> TimerState {
        let mut s = self.state.lock().unwrap();
        s.reset();
        s.clone()
    }

    pub fn get_state(&self) -> TimerState {
        self.state.lock().unwrap().clone()
    }

    pub fn start_rest(&self, settings: &Settings, is_long: bool) -> TimerState {
        let mut s = self.state.lock().unwrap();
        s.phase = if is_long { TimerPhase::LongBreak } else { TimerPhase::ShortBreak };
        let minutes = if is_long { settings.long_break } else { settings.short_break };
        s.total_secs = (minutes as u64) * 60;
        s.elapsed_secs = 0;
        s.task_id = None;
        s.clone()
    }

    pub fn skip_rest(&self) -> TimerState {
        let mut s = self.state.lock().unwrap();
        s.phase = TimerPhase::Idle;
        s.clone()
    }
}

use crate::db::Database;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn start_timer(
    app: AppHandle,
    engine: State<'_, TimerEngine>,
    db: State<'_, Database>,
    task_id: Option<i64>,
) -> Result<state::TimerState, String> {
    let settings = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row(
            "SELECT work_duration, short_break, long_break, long_break_interval,
                    auto_start_break, auto_start_work FROM settings WHERE id = 1",
            [],
            |row| Ok(config::Settings {
                work_duration: row.get(0)?,
                short_break: row.get(1)?,
                long_break: row.get(2)?,
                long_break_interval: row.get(3)?,
                auto_start_break: row.get::<_, i32>(4)? != 0,
                auto_start_work: row.get::<_, i32>(5)? != 0,
            }),
        ).map_err(|e| e.to_string())?
    };

    let result = engine.start_work(&settings, task_id);
    let engine_clone = engine.inner().clone();

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let current = engine_clone.get_state();
            if current.phase == state::TimerPhase::Idle {
                break;
            }
            if current.phase == state::TimerPhase::Paused {
                continue;
            }

            let (completed, new_state) = engine_clone.tick();
            let _ = app.emit("timer:tick", serde_json::json!({
                "phase": new_state.phase.as_str(),
                "remaining_secs": new_state.remaining_secs(),
                "total_secs": new_state.total_secs,
            }));

            if completed {
                match new_state.phase {
                    TimerPhase::Working => {
                        // Increment completed pomodoro count
                        let completed_count = {
                            let mut s = engine_clone.state.lock().unwrap();
                            s.completed_pomodoros += 1;
                            s.completed_pomodoros
                        };

                        // Save session to DB
                        if let Some(db_state) = app.try_state::<Database>() {
                            if let Ok(conn) = db_state.conn.lock() {
                                let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                let dur = new_state.total_secs.min(new_state.elapsed_secs) / 60;
                                let _ = conn.execute(
                                    "INSERT INTO pomodoro_sessions (task_id, duration, started_at, ended_at, session_type)
                                     VALUES (?1, ?2, ?3, ?4, 'work')",
                                    rusqlite::params![new_state.task_id, dur as i64, now, now],
                                );
                            }
                        }

                        let _ = app.emit("timer:completed", serde_json::json!({
                            "session_type": "work",
                            "task_id": new_state.task_id,
                        }));

                        // System notification
                        let _ = app.notification()
                            .builder()
                            .title("休息一下")
                            .body("专注完成，该休息了！")
                            .show();

                        // Auto-start break if enabled
                        if settings.auto_start_break {
                            let is_long = completed_count % settings.long_break_interval as u64 == 0;
                            engine_clone.start_rest(&settings, is_long);
                            let s = engine_clone.get_state();
                            let _ = app.emit("timer:tick", serde_json::json!({
                                "phase": s.phase.as_str(),
                                "remaining_secs": s.remaining_secs(),
                                "total_secs": s.total_secs,
                            }));
                            continue;
                        }
                    }
                    TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                        // Break completed — emit event so frontend can react
                        let _ = app.emit("timer:completed", serde_json::json!({
                            "session_type": "break",
                        }));

                        // System notification
                        let _ = app.notification()
                            .builder()
                            .title("休息一下")
                            .body("休息结束，继续加油！")
                            .show();

                        // Auto-start next work session if enabled
                        if settings.auto_start_work {
                            let task_id = { let s = engine_clone.state.lock().unwrap(); s.task_id };
                            engine_clone.start_work(&settings, task_id);
                            let s = engine_clone.get_state();
                            let _ = app.emit("timer:tick", serde_json::json!({
                                "phase": s.phase.as_str(),
                                "remaining_secs": s.remaining_secs(),
                                "total_secs": s.total_secs,
                            }));
                            continue;
                        }
                    }
                    _ => {}
                }

                engine_clone.reset();
                break;
            }
        }
    });

    Ok(result)
}

#[tauri::command]
pub fn pause_timer(
    app: AppHandle,
    engine: State<'_, TimerEngine>,
) -> Result<state::TimerState, String> {
    let result = engine.pause();
    let _ = app.emit("timer:tick", serde_json::json!({
        "phase": result.phase.as_str(),
        "remaining_secs": result.remaining_secs(),
        "total_secs": result.total_secs,
    }));
    Ok(result)
}

#[tauri::command]
pub fn resume_timer(
    app: AppHandle,
    engine: State<'_, TimerEngine>,
) -> Result<state::TimerState, String> {
    let result = engine.resume();
    let _ = app.emit("timer:tick", serde_json::json!({
        "phase": result.phase.as_str(),
        "remaining_secs": result.remaining_secs(),
        "total_secs": result.total_secs,
    }));
    Ok(result)
}

#[tauri::command]
pub fn get_timer_state(
    engine: State<'_, TimerEngine>,
    db: State<'_, Database>,
) -> Result<state::TimerState, String> {
    let mut state = engine.get_state();
    // Query today's completed pomodoros so the count survives app restart
    if let Ok(conn) = db.conn.lock() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        if let Ok(count) = conn.query_row(
            "SELECT COUNT(*) FROM pomodoro_sessions WHERE date(ended_at) = ?1 AND session_type = 'work'",
            rusqlite::params![today],
            |row| row.get::<_, i64>(0),
        ) {
            state.completed_pomodoros = count as u64;
        }
    }
    Ok(state)
}

#[tauri::command]
pub fn stop_timer(
    app: AppHandle,
    engine: State<'_, TimerEngine>,
) -> Result<state::TimerState, String> {
    let result = engine.reset();
    let _ = app.emit("timer:tick", serde_json::json!({
        "phase": "idle",
        "remaining_secs": 0,
        "total_secs": 0,
    }));
    Ok(result)
}

#[tauri::command]
pub fn skip_rest(
    app: AppHandle,
    engine: State<'_, TimerEngine>,
) -> Result<state::TimerState, String> {
    let result = engine.skip_rest();
    let _ = app.emit("timer:tick", serde_json::json!({
        "phase": "idle",
        "remaining_secs": 0,
        "total_secs": 0,
    }));
    Ok(result)
}
