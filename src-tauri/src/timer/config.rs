use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub work_duration: i64,
    pub short_break: i64,
    pub long_break: i64,
    pub long_break_interval: i64,
    pub auto_start_break: bool,
    pub auto_start_work: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work_duration: 25,
            short_break: 5,
            long_break: 15,
            long_break_interval: 4,
            auto_start_break: false,
            auto_start_work: false,
        }
    }
}
