<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useTimer } from '../composables/useTimer'
import { useTauri } from '../composables/useTauri'
import TimerDisplay from '../components/TimerDisplay.vue'
import ControlButtons from '../components/ControlButtons.vue'
import TaskQuickSelect from '../components/TaskQuickSelect.vue'

const tauri = useTauri()
const { state, init, destroy, formatTime, timerProgress, phaseLabel } = useTimer()

const currentTaskId = ref<number | null>(null)

onMounted(async () => {
  await init(tauri)
  try {
    const saved = await tauri.getTimerState() as any
    if (saved) {
      // Always sync today's completed count from DB
      if (typeof saved.completed_pomodoros === 'number') {
        state.value.completed_pomodoros = saved.completed_pomodoros
      }
      if (saved.phase !== 'idle') {
        state.value.phase = saved.phase
        state.value.remaining_secs = saved.remaining_secs ?? saved.total_secs - (saved.elapsed_secs ?? 0)
        state.value.total_secs = saved.total_secs
        state.value.task_id = saved.task_id
      }
    }
  } catch (e) { /* first launch, no state */ }
})

onUnmounted(() => destroy())

async function handleStart() {
  await tauri.startTimer(currentTaskId.value ?? undefined)
}
async function handlePause() { await tauri.pauseTimer() }
async function handleResume() { await tauri.resumeTimer() }
async function handleSkip() { await tauri.skipRest() }
async function handleStop() { await tauri.stopTimer() }

function handleTaskSelect(taskId: number | null) { currentTaskId.value = taskId }
</script>

<template>
  <div class="pomodoro-view">
    <TimerDisplay :time="formatTime(state.remaining_secs)" :phase-label="phaseLabel()" :progress="timerProgress()" />
    <ControlButtons :phase="state.phase" :start-disabled="!currentTaskId" @start="handleStart" @pause="handlePause" @resume="handleResume" @skip="handleSkip" @stop="handleStop" />
    <div class="today-summary">今日完成: <strong>{{ state.completed_pomodoros }}</strong> 个番茄钟</div>
    <TaskQuickSelect @select="handleTaskSelect" />
    <p v-if="!currentTaskId && state.phase === 'idle'" class="select-hint">请在上方选择一个任务后开始专注</p>
  </div>
</template>

<style scoped>
.pomodoro-view { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 24px; overflow-y: auto; }
.today-summary { margin-top: 20px; font-size: 14px; color: var(--color-text-muted); }
.today-summary strong { color: var(--color-primary); font-size: 18px; }
.select-hint { margin-top: 8px; font-size: 13px; color: var(--color-text-muted); }
</style>
