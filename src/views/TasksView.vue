<script setup lang="ts">
import { ref, onMounted, onActivated } from 'vue'
import type { Task } from '../types'
import { useTauri } from '../composables/useTauri'

const { listTasks, createTask, updateTask, deleteTask } = useTauri()
const tasks = ref<Task[]>([])
const showDialog = ref(false)
const newTitle = ref('')
const editingId = ref<number | null>(null)
const editTitle = ref('')

onMounted(async () => { tasks.value = await listTasks() })
onActivated(async () => { tasks.value = await listTasks() })

async function handleCreate() {
  const title = newTitle.value.trim()
  if (!title) return
  await createTask(title)
  newTitle.value = ''; showDialog.value = false
  tasks.value = await listTasks()
}

async function handleToggleComplete(task: Task) {
  await updateTask(task.id, { completed: !task.completed })
  tasks.value = await listTasks()
}

async function handleDelete(id: number) {
  await deleteTask(id)
  tasks.value = await listTasks()
}

function startEdit(task: Task) { editingId.value = task.id; editTitle.value = task.title }
async function saveEdit(id: number) {
  if (!editTitle.value.trim()) return
  await updateTask(id, { title: editTitle.value })
  editingId.value = null
  tasks.value = await listTasks()
}
function cancelEdit() { editingId.value = null; editTitle.value = '' }
</script>

<template>
  <div class="tasks-view">
    <div class="page-header">
      <h1>任务管理</h1>
      <button class="btn-add" @click="showDialog = true">+ 新建</button>
    </div>

    <div v-if="tasks.length === 0" class="empty">还没有任务，点击右上角按钮创建</div>

    <div v-else class="task-list">
      <div v-for="task in tasks" :key="task.id" :class="['task-item', { done: task.completed }]">
        <button class="check-btn" @click="handleToggleComplete(task)">{{ task.completed ? '✓' : '○' }}</button>

        <div v-if="editingId === task.id" class="edit-row">
          <input v-model="editTitle" @keyup.enter="saveEdit(task.id)" @keyup.esc="cancelEdit" />
          <button class="btn-sm" @click="saveEdit(task.id)">保存</button>
          <button class="btn-sm btn-cancel" @click="cancelEdit">取消</button>
        </div>
        <div v-else class="task-content" @dblclick="startEdit(task)">
          <span class="task-title">{{ task.title }}</span>
        </div>

        <div class="task-actions">
          <button class="btn-icon" @click="startEdit(task)" title="编辑">✎</button>
          <button class="btn-icon btn-delete" @click="handleDelete(task.id)" title="删除">✕</button>
        </div>
      </div>
    </div>

    <div v-if="showDialog" class="dialog-overlay" @click.self="showDialog = false">
      <div class="dialog">
        <h3>新建任务</h3>
        <input v-model="newTitle" placeholder="任务名称" @keyup.enter="handleCreate" />
        <div class="dialog-actions">
          <button class="btn-cancel" @click="showDialog = false">取消</button>
          <button class="btn-primary" @click="handleCreate">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tasks-view { flex: 1; padding: 24px; overflow-y: auto; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.page-header h1 { font-size: 20px; font-weight: 600; }
.btn-add { background: var(--color-primary); color: white; padding: 8px 16px; }
.empty { text-align: center; color: var(--color-text-muted); padding: 40px; }
.task-list { display: flex; flex-direction: column; gap: 6px; }
.task-item { display: flex; align-items: center; gap: 10px; background: var(--color-surface); padding: 10px 14px; border-radius: var(--radius-sm); }
.task-item.done { opacity: 0.5; }
.task-item.done .task-title { text-decoration: line-through; }
.check-btn { background: none; color: var(--color-text-muted); font-size: 18px; width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; }
.task-content { flex: 1; min-width: 0; cursor: default; }
.task-title { font-size: 14px; }
.edit-row { flex: 1; display: flex; gap: 6px; }
.edit-row input { flex: 1; }
.btn-sm { padding: 4px 12px; background: var(--color-primary); color: white; font-size: 12px; }
.btn-cancel { background: transparent; color: var(--color-text-muted); }
.task-actions { display: flex; gap: 4px; }
.btn-icon { background: none; color: var(--color-text-muted); font-size: 14px; padding: 4px; width: 28px; height: 28px; opacity: 0; }
.task-item:hover .btn-icon { opacity: 1; }
.btn-icon:hover { color: var(--color-text); }
.btn-delete:hover { color: var(--color-primary); }
.dialog-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; }
.dialog { background: var(--color-surface); padding: 24px; border-radius: var(--radius); width: 320px; display: flex; flex-direction: column; gap: 16px; }
.dialog h3 { font-size: 16px; }
.dialog-actions { display: flex; justify-content: flex-end; gap: 8px; }
.dialog-actions button { padding: 8px 20px; }
.dialog-actions .btn-primary { background: var(--color-primary); color: white; }
</style>
