<script setup lang="ts">
import { ref, computed, onMounted, onActivated } from 'vue'
import type { Task } from '../types'
import { useTauri } from '../composables/useTauri'

const { listTasks, createTask } = useTauri()

const tasks = ref<Task[]>([])
const selectedTaskId = ref<number | null>(null)
const selectedTaskTitle = ref('')
const query = ref('')
const showDropdown = ref(false)

const emit = defineEmits<{ select: [taskId: number | null, taskTitle?: string] }>()

const filteredTasks = computed(() => {
  if (!query.value.trim()) return tasks.value
  return tasks.value.filter(t => t.title.includes(query.value.trim()))
})

onMounted(async () => { tasks.value = (await listTasks()).filter(t => !t.completed) })
onActivated(async () => { tasks.value = (await listTasks()).filter(t => !t.completed) })

async function handleEnter() {
  const text = query.value.trim()
  if (!text) return
  const match = tasks.value.find(t => t.title === text)
  if (match) { selectTask(match); return }
  const task = await createTask(text)
  tasks.value.unshift(task)
  selectTask(task)
}

function selectTask(task: Task) {
  selectedTaskId.value = task.id
  selectedTaskTitle.value = task.title
  query.value = ''
  showDropdown.value = false
  emit('select', task.id, task.title)
}

function clearSelection() {
  selectedTaskId.value = null
  selectedTaskTitle.value = ''
  emit('select', null)
}

function onBlur() {
  setTimeout(() => { showDropdown.value = false }, 200)
}
</script>

<template>
  <div class="task-combo">
    <div v-if="selectedTaskId" class="selected-pill">
      <span class="check">✓</span>
      <span class="title">{{ selectedTaskTitle }}</span>
      <button class="clear-btn" @click="clearSelection">✕</button>
    </div>
    <div v-else class="combo-wrapper">
      <input v-model="query" placeholder="专注什么？" @focus="showDropdown = true" @blur="onBlur" @keyup.enter="handleEnter" />
      <div v-if="showDropdown && filteredTasks.length > 0" class="dropdown">
        <button v-for="t in filteredTasks" :key="t.id" class="dropdown-item" @mousedown.prevent="selectTask(t)">
          <span class="item-title">{{ t.title }}</span>
          <span v-if="t.session_count > 0" class="item-count">{{ t.session_count }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.task-combo { margin-top: 16px; width: 100%; max-width: 360px; }
.combo-wrapper { position: relative; }
.combo-wrapper input { width: 100%; }
.dropdown { position: absolute; top: 100%; left: 0; right: 0; margin-top: 4px; background: var(--color-surface); border-radius: var(--radius-sm); box-shadow: 0 4px 12px rgba(0,0,0,0.15); z-index: 10; max-height: 160px; overflow-y: auto; }
.dropdown-item { display: flex; align-items: center; gap: 8px; width: 100%; padding: 10px 12px; text-align: left; background: none; color: var(--color-text); border-radius: 0; font-size: 13px; }
.dropdown-item:first-child { border-radius: var(--radius-sm) var(--radius-sm) 0 0; }
.dropdown-item:last-child { border-radius: 0 0 var(--radius-sm) var(--radius-sm); }
.dropdown-item:hover { background: var(--color-secondary); }
.item-title { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.item-count { background: var(--color-secondary); color: var(--color-text-muted); padding: 1px 7px; border-radius: 10px; font-size: 11px; flex-shrink: 0; }
.selected-pill { display: flex; align-items: center; gap: 8px; padding: 10px 16px; border-radius: var(--radius-sm); background: var(--color-surface); font-size: 14px; }
.selected-pill .check { color: var(--color-success); font-size: 16px; }
.selected-pill .title { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.selected-pill .clear-btn { background: none; color: var(--color-text-muted); font-size: 14px; padding: 2px 6px; border-radius: 4px; line-height: 1; }
.selected-pill .clear-btn:hover { color: var(--color-text); background: var(--color-secondary); }
</style>
