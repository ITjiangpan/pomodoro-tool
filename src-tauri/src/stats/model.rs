use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total_pomodoros: i64,
    pub total_focus_minutes: i64,
    pub completed_tasks: i64,
    pub daily_data: Vec<DailyCount>,
    pub top_tasks: Vec<TaskCount>,
}

#[derive(Debug, Serialize)]
pub struct DailyCount {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct TaskCount {
    pub title: String,
    pub count: i64,
}
