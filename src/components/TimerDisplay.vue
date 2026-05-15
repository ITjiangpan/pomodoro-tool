<script setup lang="ts">
defineProps<{ time: string; phaseLabel: string; progress: number; phase: string }>()
</script>

<template>
  <div class="timer-display">
    <svg class="progress-ring" viewBox="0 0 200 200">
      <circle class="bg-ring" cx="100" cy="100" r="88" />
      <circle v-if="phase !== 'idle'" class="fg-ring" :class="phase" cx="100" cy="100" r="88"
        :style="{
          strokeDasharray: `${2 * Math.PI * 88}`,
          strokeDashoffset: `${2 * Math.PI * 88 * (1 - progress)}`,
        }"
      />
    </svg>
    <div class="timer-content">
      <template v-if="phase === 'idle'">
        <div class="ready-icon">🍅</div>
        <div class="ready-text">准备开始</div>
      </template>
      <template v-else>
        <div class="time-text">{{ time }}</div>
        <div class="phase-text">{{ phaseLabel }}</div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.timer-display { position: relative; width: 240px; height: 240px; margin: 0 auto; }
.progress-ring { width: 100%; height: 100%; transform: rotate(-90deg); }
.bg-ring { fill: none; stroke: var(--color-secondary); stroke-width: 6; }
.fg-ring { fill: none; stroke: var(--color-primary); stroke-width: 6; stroke-linecap: round; transition: stroke-dashoffset 0.5s ease, stroke 0.3s ease; }
.fg-ring.short_break,
.fg-ring.long_break { stroke: #52b788; }
.fg-ring.paused { stroke: var(--color-text-muted); }
.timer-content { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; }
.ready-icon { font-size: 48px; margin-bottom: 8px; }
.ready-text { font-size: 18px; color: var(--color-text-muted); }
.time-text { font-size: 56px; font-weight: 700; letter-spacing: 2px; font-variant-numeric: tabular-nums; }
.phase-text { font-size: 14px; color: var(--color-text-muted); margin-top: 4px; }
</style>
