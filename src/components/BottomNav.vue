<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()

const tabs = [
  { path: '/', icon: '🍅', label: '番茄钟' },
  { path: '/tasks', icon: '📋', label: '任务' },
  { path: '/stats', icon: '📊', label: '统计' },
  { path: '/settings', icon: '⚙', label: '设置' },
]
</script>

<template>
  <nav class="bottom-nav">
    <button
      v-for="tab in tabs" :key="tab.path"
      :class="['nav-item', { active: route.path === tab.path }]"
      @click="router.push(tab.path)"
    >
      <span class="nav-icon">{{ tab.icon }}</span>
      <span class="nav-label">{{ tab.label }}</span>
    </button>
  </nav>
</template>

<style scoped>
.bottom-nav {
  display: flex; justify-content: space-around; align-items: center;
  background: var(--color-surface); border-top: 1px solid rgba(255,255,255,0.06);
  padding: 8px 0; flex-shrink: 0;
}
.nav-item {
  display: flex; flex-direction: column; align-items: center; gap: 2px;
  background: none; color: var(--color-text-muted); padding: 4px 16px;
  border-radius: var(--radius-sm); font-size: 11px;
}
.nav-item.active { color: var(--color-primary); }
.nav-icon { font-size: 20px; line-height: 1; }
.nav-label { font-size: 10px; }
</style>
