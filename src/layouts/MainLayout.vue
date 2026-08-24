<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { checkAuth, logout } from '../services/auth'
import { checkUpdate } from '../services/github'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const username = ref('')

onMounted(async () => {
  username.value = (await checkAuth()) ?? ''
  const update = await checkUpdate().catch(() => null)
  if (update && confirm(`Новая версия ${update.version}. Обновиться?`)) {
    await invoke('apply_update', { url: update.download_url })
  }
})

async function doLogout() { await logout(); router.push('/login') }
</script>

<template>
  <div class="layout">
    <aside class="sidebar">
      <div class="logo"><h1>XQC</h1><p>Game Launcher</p></div>
      <nav>
        <router-link to="/games" class="nav-btn">🎮 Игры</router-link>
        <router-link to="/mods" class="nav-btn">🛒 Моды</router-link>
        <router-link to="/scripts" class="nav-btn">📜 Script Hub</router-link>
        <router-link to="/settings" class="nav-btn">⚙️ Настройки</router-link>
      </nav>
      <div class="footer">
        <span class="user">👤 {{ username }}</span>
        <button class="btn-logout" @click="doLogout">Выйти</button>
      </div>
    </aside>
    <main class="content"><router-view /></main>
  </div>
</template>

<style scoped>
.layout { display: flex; height: 100vh; }
.sidebar { width: 260px; background: var(--bg-dark); display: flex; flex-direction: column; border-right: 1px solid #2a3f5a; }
.logo { padding: 25px 20px; text-align: center; border-bottom: 1px solid #2a3f5a; }
.logo h1 { color: var(--accent); letter-spacing: 4px; font-size: 28px; }
.logo p { color: var(--text-dim); font-size: 12px; }
nav { flex: 1; padding: 20px 0; display: flex; flex-direction: column; }
.nav-btn { color: var(--text-dim); padding: 15px 25px; text-decoration: none; font-size: 15px; }
.nav-btn:hover { background: rgba(42,63,90,.5); color: var(--text); }
.nav-btn.router-link-active { background: rgba(42,63,90,.8); color: var(--accent); border-left: 3px solid var(--accent); }
.footer { padding: 20px; border-top: 1px solid #2a3f5a; display: flex; justify-content: space-between; align-items: center; }
.user { color: var(--accent); font-weight: bold; }
.btn-logout { background: transparent; color: var(--red); border: 1px solid var(--red); padding: 6px 12px; }
.btn-logout:hover { background: var(--red); color: white; }
.content { flex: 1; overflow-y: auto; padding: 30px; }
</style>