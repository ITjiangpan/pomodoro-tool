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
