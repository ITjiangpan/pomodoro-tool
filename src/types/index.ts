export interface TimerState {
  phase: 'idle' | 'working' | 'short_break' | 'long_break' | 'paused'
  remaining_secs: number
  total_secs: number
  current_session_id: number | null
  task_id: number | null
  completed_pomodoros: number
}

export interface TimerTick {
  phase: TimerState['phase']
  remaining_secs: number
  total_secs: number
}

export interface TimerPhaseChange {
  from: string
  to: string
}

export interface Task {
  id: number
  title: string
  description: string
  completed: boolean
  created_at: string
  session_count: number
  last_used_at: string | null
}

export interface Settings {
  work_duration: number
  short_break: number
  long_break: number
  long_break_interval: number
  auto_start_break: boolean
  auto_start_work: boolean
  theme: string
}

export interface Stats {
  total_pomodoros: number
  total_focus_minutes: number
  completed_tasks: number
  daily_data: { date: string; count: number }[]
  top_tasks: { title: string; count: number }[]
}
