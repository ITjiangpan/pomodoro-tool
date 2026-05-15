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
const currentTaskTitle = ref('')
const showStopConfirm = ref(false)
const showSkipConfirm = ref(false)

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
function handleSkip() { showSkipConfirm.value = true }
async function confirmSkip() {
  showSkipConfirm.value = false
  await tauri.skipRest()
}
function handleStop() { showStopConfirm.value = true }
async function confirmStop() {
  showStopConfirm.value = false
  await tauri.stopTimer()
}
async function cancelStop() {
  showStopConfirm.value = false
  await tauri.resumeTimer()
}

function handleTaskSelect(taskId: number | null, taskTitle?: string) {
  currentTaskId.value = taskId
  if (taskTitle) currentTaskTitle.value = taskTitle
}
</script>

<template>
  <div class="pomodoro-view">
    <TimerDisplay :time="formatTime(state.remaining_secs)" :phase-label="phaseLabel()" :progress="timerProgress()" :phase="state.phase" />
    <ControlButtons :phase="state.phase" :start-disabled="!currentTaskId" @start="handleStart" @pause="handlePause" @resume="handleResume" @skip="handleSkip" @stop="handleStop" />
    <div class="today-summary">今日完成: <strong>{{ state.completed_pomodoros }}</strong> 个番茄钟</div>
    <template v-if="state.phase === 'idle'">
      <TaskQuickSelect @select="handleTaskSelect" />
    </template>
    <div v-else-if="currentTaskId" class="current-task-bar">
      <div class="section-title">当前任务</div>
      <div class="task-label">
        <span class="task-check">✓</span>
        {{ currentTaskTitle || '任务 #' + currentTaskId }}
      </div>
    </div>

    <Teleport to="body">
      <div v-if="showStopConfirm" class="confirm-overlay" @click.self="showStopConfirm = false">
        <div class="confirm-dialog">
          <h3>确定放弃当前番茄钟？</h3>
          <p>本次专注将不会记录到统计中</p>
          <div class="confirm-actions">
            <button class="btn-cancel" @click="cancelStop">继续专注</button>
            <button class="btn-danger" @click="confirmStop">放弃</button>
          </div>
        </div>
      </div>
      <div v-if="showSkipConfirm" class="confirm-overlay" @click.self="showSkipConfirm = false">
        <div class="confirm-dialog">
          <h3>确定跳过本次休息？</h3>
          <p>休息太短，效果可能不好哦</p>
          <div class="confirm-actions">
            <button class="btn-cancel" @click="showSkipConfirm = false">取消</button>
            <button class="btn-danger" @click="confirmSkip">跳过</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.pomodoro-view { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 24px; overflow-y: auto; }
.today-summary { margin-top: 20px; font-size: 14px; color: var(--color-text-muted); }
.today-summary strong { color: var(--color-primary); font-size: 18px; }
.confirm-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 100; }
.confirm-dialog { background: var(--color-surface); border-radius: var(--radius); padding: 24px; width: 300px; text-align: center; }
.confirm-dialog h3 { font-size: 16px; margin-bottom: 8px; }
.confirm-dialog p { font-size: 13px; color: var(--color-text-muted); margin-bottom: 20px; }
.confirm-actions { display: flex; gap: 12px; justify-content: center; }
.confirm-actions button { padding: 8px 24px; border-radius: 50px; font-size: 14px; }
.btn-cancel { background: var(--color-secondary); color: var(--color-text); }
.btn-danger { background: var(--color-primary); color: white; }
.current-task-bar { margin-top: 20px; width: 100%; max-width: 360px; text-align: center; }
.current-task-bar .section-title { font-size: 12px; color: var(--color-text-muted); margin-bottom: 8px; text-transform: uppercase; letter-spacing: 1px; }
.current-task-bar .task-label { background: var(--color-surface); padding: 10px 16px; border-radius: var(--radius-sm); font-size: 14px; display: flex; align-items: center; justify-content: center; gap: 8px; }
.current-task-bar .task-check { color: var(--color-success); font-size: 16px; }
</style>
