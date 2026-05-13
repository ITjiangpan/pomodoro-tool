<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { Settings } from '../types'
import { useTauri } from '../composables/useTauri'

const { getSettings, updateSettings } = useTauri()

const settings = ref<Settings>({
  work_duration: 25, short_break: 5, long_break: 15,
  long_break_interval: 4, auto_start_break: false, auto_start_work: false,
})

const saved = ref(false)

onMounted(async () => { settings.value = await getSettings() })

async function save() {
  await updateSettings(settings.value)
  saved.value = true
  setTimeout(() => { saved.value = false }, 2000)
}
</script>

<template>
  <div class="settings-view">
    <div class="page-header">
      <h1>设置</h1>
      <button class="btn-save" @click="save">{{ saved ? '已保存 ✓' : '保存' }}</button>
    </div>

    <section>
      <div class="section-title">番茄钟时长</div>
      <div class="setting-row">
        <label>工作</label>
        <div class="input-group"><input type="number" v-model.number="settings.work_duration" min="1" max="120" /><span>分钟</span></div>
      </div>
      <div class="setting-row">
        <label>短休息</label>
        <div class="input-group"><input type="number" v-model.number="settings.short_break" min="1" max="30" /><span>分钟</span></div>
      </div>
      <div class="setting-row">
        <label>长休息</label>
        <div class="input-group"><input type="number" v-model.number="settings.long_break" min="1" max="60" /><span>分钟</span></div>
      </div>
      <div class="setting-row">
        <label>长休息间隔</label>
        <div class="input-group"><input type="number" v-model.number="settings.long_break_interval" min="2" max="10" /><span>个番茄钟</span></div>
      </div>
    </section>

    <section>
      <div class="section-title">行为偏好</div>
      <div class="setting-row toggle-row">
        <label>工作完成后自动开始休息</label>
        <label class="toggle"><input type="checkbox" v-model="settings.auto_start_break" /><span class="slider"></span></label>
      </div>
      <div class="setting-row toggle-row">
        <label>休息完成后自动开始工作</label>
        <label class="toggle"><input type="checkbox" v-model="settings.auto_start_work" /><span class="slider"></span></label>
      </div>
    </section>

    <section>
      <div class="section-title">数据管理</div>
      <div class="setting-row"><label>所有数据存储在本地，不会上传到网络</label></div>
    </section>
  </div>
</template>

<style scoped>
.settings-view { flex: 1; padding: 24px; overflow-y: auto; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.page-header h1 { font-size: 20px; font-weight: 600; }
.btn-save { background: var(--color-primary); color: white; padding: 8px 20px; }
section { margin-bottom: 28px; }
.section-title { font-size: 12px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 1px; margin-bottom: 12px; }
.setting-row { display: flex; justify-content: space-between; align-items: center; background: var(--color-surface); padding: 12px 16px; border-radius: var(--radius-sm); margin-bottom: 6px; }
.setting-row label { font-size: 14px; }
.input-group { display: flex; align-items: center; gap: 8px; }
.input-group input[type="number"] { width: 64px; text-align: center; }
.input-group span { font-size: 13px; color: var(--color-text-muted); }
.toggle-row label:first-child { flex: 1; }
.toggle { position: relative; display: inline-block; width: 44px; height: 24px; cursor: pointer; }
.toggle input { opacity: 0; width: 0; height: 0; }
.slider { position: absolute; inset: 0; background: var(--color-secondary); border-radius: 24px; transition: 0.3s; }
.slider::before { content: ''; position: absolute; width: 20px; height: 20px; left: 2px; bottom: 2px; background: white; border-radius: 50%; transition: 0.3s; }
.toggle input:checked + .slider { background: var(--color-primary); }
.toggle input:checked + .slider::before { transform: translateX(20px); }
</style>
