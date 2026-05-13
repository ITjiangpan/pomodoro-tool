import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Task, Settings, Stats, TimerTick } from '../types'

export function useTauri() {
  async function createTask(title: string, description?: string): Promise<Task> {
    return invoke('create_task', { request: { title, description: description ?? '' } })
  }
  async function updateTask(id: number, data: { title?: string; completed?: boolean }): Promise<Task> {
    return invoke('update_task', { request: { id, ...data } })
  }
  async function deleteTask(id: number): Promise<void> {
    return invoke('delete_task', { id })
  }
  async function listTasks(): Promise<Task[]> {
    return invoke('list_tasks')
  }
  async function getSettings(): Promise<Settings> {
    return invoke('get_settings')
  }
  async function updateSettings(settings: Settings): Promise<Settings> {
    return invoke('update_settings', { settings })
  }
  async function getStats(range: 'daily' | 'weekly' | 'monthly'): Promise<Stats> {
    return invoke('get_stats', { range })
  }

  async function startTimer(taskId?: number) {
    return invoke('start_timer', { taskId: taskId ?? null })
  }
  async function pauseTimer() {
    return invoke('pause_timer')
  }
  async function resumeTimer() {
    return invoke('resume_timer')
  }
  async function stopTimer() {
    return invoke('stop_timer')
  }
  async function skipRest() {
    return invoke('skip_rest')
  }
  async function getTimerState() {
    return invoke('get_timer_state')
  }

  function onTimerTick(callback: (payload: TimerTick) => void) {
    return listen<TimerTick>('timer:tick', (event) => callback(event.payload))
  }
  function onTimerCompleted(callback: (payload: { session_type: string; task_id: number | null }) => void) {
    return listen('timer:completed', (event) => callback(event.payload as any))
  }

  return {
    createTask, updateTask, deleteTask, listTasks,
    getSettings, updateSettings,
    getStats,
    startTimer, pauseTimer, resumeTimer, stopTimer, skipRest, getTimerState,
    onTimerTick, onTimerCompleted,
  }
}
