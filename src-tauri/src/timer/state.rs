use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum TimerPhase {
    Idle,
    Working,
    ShortBreak,
    LongBreak,
    Paused,
}

impl TimerPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimerPhase::Idle => "idle",
            TimerPhase::Working => "working",
            TimerPhase::ShortBreak => "short_break",
            TimerPhase::LongBreak => "long_break",
            TimerPhase::Paused => "paused",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimerState {
    pub phase: TimerPhase,
    pub elapsed_secs: u64,
    pub total_secs: u64,
    pub current_session_id: Option<i64>,
    pub task_id: Option<i64>,
    pub completed_pomodoros: u64,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            phase: TimerPhase::Idle,
            elapsed_secs: 0,
            total_secs: 0,
            current_session_id: None,
            task_id: None,
            completed_pomodoros: 0,
        }
    }

    pub fn remaining_secs(&self) -> u64 {
        self.total_secs.saturating_sub(self.elapsed_secs)
    }

    pub fn progress(&self) -> f64 {
        if self.total_secs == 0 {
            return 0.0;
        }
        self.elapsed_secs as f64 / self.total_secs as f64
    }

    /// Returns true when timer completes (elapsed >= total)
    pub fn tick(&mut self) -> bool {
        if self.phase == TimerPhase::Paused || self.phase == TimerPhase::Idle {
            return false;
        }
        self.elapsed_secs += 1;
        self.elapsed_secs >= self.total_secs
    }

    pub fn reset(&mut self) {
        self.phase = TimerPhase::Idle;
        self.elapsed_secs = 0;
        self.total_secs = 0;
        self.current_session_id = None;
        self.task_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_timer_is_idle() {
        let state = TimerState::new();
        assert_eq!(state.phase, TimerPhase::Idle);
        assert_eq!(state.remaining_secs(), 0);
    }

    #[test]
    fn test_tick_idle_does_nothing() {
        let mut state = TimerState::new();
        assert!(!state.tick());
        assert_eq!(state.elapsed_secs, 0);
    }

    #[test]
    fn test_tick_increments_and_signals_completion() {
        let mut state = TimerState::new();
        state.phase = TimerPhase::Working;
        state.total_secs = 3;
        state.elapsed_secs = 0;

        assert!(!state.tick()); // 1/3
        assert!(!state.tick()); // 2/3
        assert!(state.tick());  // 3/3 → completed
    }

    #[test]
    fn test_paused_tick_does_nothing() {
        let mut state = TimerState::new();
        state.phase = TimerPhase::Paused;
        state.total_secs = 10;
        assert!(!state.tick());
        assert_eq!(state.elapsed_secs, 0);
    }

    #[test]
    fn test_progress() {
        let mut state = TimerState::new();
        state.phase = TimerPhase::Working;
        state.total_secs = 100;
        state.elapsed_secs = 25;
        assert!((state.progress() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_reset() {
        let mut state = TimerState::new();
        state.phase = TimerPhase::Working;
        state.elapsed_secs = 50;
        state.total_secs = 100;
        state.current_session_id = Some(1);
        state.task_id = Some(42);
        state.reset();
        assert_eq!(state.phase, TimerPhase::Idle);
        assert_eq!(state.elapsed_secs, 0);
        assert_eq!(state.current_session_id, None);
        assert_eq!(state.task_id, None);
    }
}
