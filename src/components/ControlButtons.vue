<script setup lang="ts">
defineProps<{ phase: string }>()
const emit = defineEmits<{ start: []; pause: []; resume: []; skip: []; stop: [] }>()
</script>

<template>
  <div class="controls">
    <template v-if="phase === 'idle'">
      <button class="btn btn-primary" @click="emit('start')">开始专注</button>
    </template>
    <template v-else-if="phase === 'working'">
      <button class="btn btn-secondary" @click="emit('pause')">暂停</button>
    </template>
    <template v-else-if="phase === 'paused'">
      <button class="btn btn-primary" @click="emit('resume')">继续</button>
      <button class="btn btn-ghost" @click="emit('stop')">放弃</button>
    </template>
    <template v-else-if="phase === 'short_break' || phase === 'long_break'">
      <button class="btn btn-secondary" @click="emit('skip')">跳过休息</button>
    </template>
  </div>
</template>

<style scoped>
.controls { display: flex; gap: 12px; justify-content: center; margin-top: 24px; }
.btn { padding: 12px 32px; border-radius: 50px; font-size: 16px; font-weight: 600; }
.btn-primary { background: var(--color-primary); color: white; }
.btn-primary:hover { background: #d63851; }
.btn-secondary { background: var(--color-secondary); color: var(--color-text); }
.btn-secondary:hover { background: #1a4a7a; }
.btn-ghost { background: transparent; color: var(--color-text-muted); border: 1px solid var(--color-text-muted); }
.btn-ghost:hover { color: var(--color-primary); border-color: var(--color-primary); }
</style>
