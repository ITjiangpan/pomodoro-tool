<script setup lang="ts">
import { ref, onMounted, onActivated } from 'vue'
import type { Task } from '../types'
import { useTauri } from '../composables/useTauri'

const { listTasks, createTask } = useTauri()

const tasks = ref<Task[]>([])
const selectedTaskId = ref<number | null>(null)
const newTaskTitle = ref('')
const loading = ref(false)

const emit = defineEmits<{ select: [taskId: number | null] }>()

onMounted(async () => { tasks.value = (await listTasks()).filter(t => !t.completed) })

// Refresh task list when navigating back to this page
onActivated(async () => { tasks.value = (await listTasks()).filter(t => !t.completed) })

async function handleCreate() {
  const title = newTaskTitle.value.trim()
  if (!title) return
  loading.value = true
  try {
    const task = await createTask(title)
    newTaskTitle.value = ''
    tasks.value.unshift(task)
    selectedTaskId.value = task.id
    emit('select', task.id)
  } finally { loading.value = false }
}

function handleSelect(id: number) {
  selectedTaskId.value = id
  emit('select', id)
}
</script>

<template>
  <div class="task-qs">
    <div class="section-title">选择任务 <span class="required">*必选</span></div>
    <div class="task-list">
      <button v-for="t in tasks" :key="t.id"
        :class="['task-opt', { selected: selectedTaskId === t.id }]"
        @click="handleSelect(t.id)"
      >{{ t.title }}</button>
      <div v-if="tasks.length === 0 && !newTaskTitle" class="empty-hint">
        还没有任务，输入名称创建第一个
      </div>
    </div>
    <div class="create-row">
      <input v-model="newTaskTitle" placeholder="输入任务名称..." @keyup.enter="handleCreate" />
      <button :disabled="!newTaskTitle.trim() || loading" @click="handleCreate">添加</button>
    </div>
  </div>
</template>

<style scoped>
.task-qs { margin-top: 16px; width: 100%; max-width: 360px; }
.section-title { font-size: 12px; color: var(--color-text-muted); margin-bottom: 8px; text-transform: uppercase; letter-spacing: 1px; }
.required { color: var(--color-primary); font-weight: 400; letter-spacing: 0; margin-left: 4px; }
.task-list { display: flex; flex-direction: column; gap: 4px; margin-bottom: 12px; max-height: 120px; overflow-y: auto; }
.task-opt { background: var(--color-surface); color: var(--color-text); padding: 8px 12px; text-align: left; border-radius: var(--radius-sm); font-size: 13px; }
.task-opt:hover { background: var(--color-secondary); }
.task-opt.selected { background: var(--color-primary); color: white; }
.empty-hint { color: var(--color-text-muted); font-size: 13px; text-align: center; padding: 12px; }
.create-row { display: flex; gap: 8px; }
.create-row input { flex: 1; }
.create-row button { background: var(--color-primary); color: white; padding: 8px 16px; }
.create-row button:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
