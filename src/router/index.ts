import { createRouter, createWebHistory } from 'vue-router'
import PomodoroView from '../views/PomodoroView.vue'

const routes = [
  { path: '/', name: 'pomodoro', component: PomodoroView },
  { path: '/tasks', name: 'tasks', component: () => import('../views/TasksView.vue') },
  { path: '/stats', name: 'stats', component: () => import('../views/StatsView.vue') },
  { path: '/settings', name: 'settings', component: () => import('../views/SettingsView.vue') },
]

export default createRouter({
  history: createWebHistory(),
  routes,
})
