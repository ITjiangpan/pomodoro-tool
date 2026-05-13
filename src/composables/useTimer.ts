import { ref } from 'vue'
import type { TimerState, TimerTick } from '../types'

export function useTimer() {
  const state = ref<TimerState>({
    phase: 'idle',
    remaining_secs: 0,
    total_secs: 0,
    current_session_id: null,
    task_id: null,
    completed_pomodoros: 0,
  })

  let unlistenTick: (() => void) | null = null
  let unlistenComplete: (() => void) | null = null

  async function init(listen: {
    onTimerTick: (cb: (t: TimerTick) => void) => Promise<() => void>
    onTimerCompleted: (cb: (p: any) => void) => Promise<() => void>
  }) {
    unlistenTick = await listen.onTimerTick((tick: TimerTick) => {
      state.value.remaining_secs = tick.remaining_secs
      state.value.total_secs = tick.total_secs
      state.value.phase = tick.phase
    })
    unlistenComplete = await listen.onTimerCompleted(() => {
      state.value.completed_pomodoros++
    })
  }

  function destroy() {
    unlistenTick?.()
    unlistenComplete?.()
  }

  function formatTime(secs: number): string {
    const m = Math.floor(secs / 60)
    const s = secs % 60
    return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
  }

  function timerProgress(): number {
    if (state.value.total_secs === 0) return 0
    return 1 - state.value.remaining_secs / state.value.total_secs
  }

  function phaseLabel(): string {
    const map: Record<string, string> = {
      idle: '准备就绪', working: '专注中', short_break: '短休息',
      long_break: '长休息', paused: '已暂停',
    }
    return map[state.value.phase] || state.value.phase
  }

  return { state, init, destroy, formatTime, timerProgress, phaseLabel }
}
