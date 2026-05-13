<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import type { Stats } from '../types'
import { useTauri } from '../composables/useTauri'

const { getStats } = useTauri()
const range = ref<'daily' | 'weekly' | 'monthly'>('daily')
const stats = ref<Stats | null>(null)
const loading = ref(true)

async function loadStats() {
  loading.value = true
  try { stats.value = await getStats(range.value) }
  finally { loading.value = false }
}

onMounted(loadStats)
watch(range, loadStats)

function maxCount(): number {
  if (!stats.value?.daily_data.length) return 1
  return Math.max(...stats.value.daily_data.map(d => d.count), 1)
}
</script>

<template>
  <div class="stats-view">
    <div class="page-header">
      <h1>统计</h1>
      <div class="range-tabs">
        <button :class="{ active: range === 'daily' }" @click="range = 'daily'">日</button>
        <button :class="{ active: range === 'weekly' }" @click="range = 'weekly'">周</button>
        <button :class="{ active: range === 'monthly' }" @click="range = 'monthly'">月</button>
      </div>
    </div>

    <div v-if="loading" class="empty">加载中...</div>

    <template v-else-if="!stats || stats.total_pomodoros === 0">
      <div class="empty">完成第一个番茄钟后，这里会出现统计</div>
    </template>

    <template v-else>
      <div class="summary-cards">
        <div class="card"><div class="card-value">{{ stats.total_pomodoros }}</div><div class="card-label">番茄钟</div></div>
        <div class="card"><div class="card-value">{{ Math.floor(stats.total_focus_minutes / 60) }}h{{ stats.total_focus_minutes % 60 }}m</div><div class="card-label">专注时长</div></div>
        <div class="card"><div class="card-value">{{ stats.completed_tasks }}</div><div class="card-label">完成任务</div></div>
      </div>

      <div class="chart-section">
        <div class="section-title">每日番茄</div>
        <div class="bar-chart">
          <div v-for="d in stats.daily_data" :key="d.date" class="bar-wrapper">
            <div class="bar" :style="{ height: (d.count / maxCount() * 120) + 'px' }"></div>
            <div class="bar-label">{{ d.date.slice(5) }}</div>
            <div class="bar-value">{{ d.count }}</div>
          </div>
        </div>
      </div>

      <div v-if="stats.top_tasks.length" class="top-tasks">
        <div class="section-title">热门任务</div>
        <div v-for="(t, i) in stats.top_tasks" :key="i" class="task-row">
          <span class="task-rank">{{ i + 1 }}</span>
          <span class="task-name">{{ t.title }}</span>
          <span class="task-count">{{ t.count }} 🍅</span>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.stats-view { flex: 1; padding: 24px; overflow-y: auto; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.page-header h1 { font-size: 20px; font-weight: 600; }
.range-tabs { display: flex; gap: 4px; background: var(--color-surface); border-radius: var(--radius-sm); padding: 3px; }
.range-tabs button { background: none; color: var(--color-text-muted); padding: 6px 14px; font-size: 13px; border-radius: 6px; }
.range-tabs button.active { background: var(--color-primary); color: white; }
.empty { text-align: center; color: var(--color-text-muted); padding: 60px 20px; }
.summary-cards { display: flex; gap: 12px; margin-bottom: 24px; }
.card { flex: 1; background: var(--color-surface); border-radius: var(--radius); padding: 16px; text-align: center; }
.card-value { font-size: 24px; font-weight: 700; color: var(--color-primary); }
.card-label { font-size: 12px; color: var(--color-text-muted); margin-top: 4px; }
.chart-section { margin-bottom: 24px; }
.section-title { font-size: 12px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 1px; margin-bottom: 12px; }
.bar-chart { display: flex; align-items: flex-end; gap: 8px; height: 160px; padding: 8px 0; }
.bar-wrapper { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4px; height: 100%; justify-content: flex-end; }
.bar { width: 100%; max-width: 36px; background: var(--color-primary); border-radius: 4px 4px 0 0; min-height: 4px; }
.bar-label { font-size: 10px; color: var(--color-text-muted); }
.bar-value { font-size: 10px; color: var(--color-text); font-weight: 600; }
.task-row { display: flex; align-items: center; gap: 10px; background: var(--color-surface); padding: 10px 14px; border-radius: var(--radius-sm); margin-bottom: 4px; }
.task-rank { color: var(--color-text-muted); font-size: 12px; width: 16px; }
.task-name { flex: 1; font-size: 14px; }
.task-count { color: var(--color-accent); font-size: 13px; }
</style>
