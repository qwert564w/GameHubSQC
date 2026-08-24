import { createRouter, createWebHashHistory } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/login', name: 'login', component: () => import('../views/LoginView.vue') },
    {
      path: '/',
      component: () => import('../layouts/MainLayout.vue'),
      children: [
        { path: '', redirect: '/games' },
        { path: 'games', name: 'games', component: () => import('../views/GamesView.vue') },
        { path: 'mods', name: 'mods', component: () => import('../views/ModsView.vue') },
        { path: 'settings', name: 'settings', component: () => import('../views/SettingsView.vue') },
        { path: 'scripts', name: 'scripts', component: () => import('../views/ScriptHubView.vue') },
      ],
    },
  ],
})

router.beforeEach(async (to) => {
  const user = await invoke<string | null>('check_auth').catch(() => null)
  if (!user && to.name !== 'login') return { name: 'login' }
  if (user && to.name === 'login') return { path: '/games' }
})

export default router