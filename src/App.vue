<script setup lang="ts">
import { onMounted } from 'vue'
import BottomNav from './components/BottomNav.vue'
import { useTauri } from './composables/useTauri'

const { getSettings } = useTauri()

onMounted(async () => {
  try {
    const settings = await getSettings()
    document.documentElement.dataset.theme = settings.theme
  } catch (e) {
    document.documentElement.dataset.theme = 'light'
  }
})
</script>

<template>
  <div class="app-shell">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
    <BottomNav />
  </div>
</template>

<style scoped>
.app-shell { display: flex; flex-direction: column; height: 100vh; }
</style>
