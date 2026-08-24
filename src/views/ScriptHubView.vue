<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Script { name: string; path: string; lang: string }
const scripts = ref<Script[]>([])
const scriptsPath = ref('')
const status = ref('')

onMounted(async () => {
  scriptsPath.value = await invoke<string>('get_scripts_dir')
  scripts.value = await invoke<Script[]>('list_scripts')
})

async function openFolder() { await invoke('open_scripts_folder') }
async function runScript(s: Script) {
  status.value = `▶ Запуск ${s.name}...`
  try { await invoke<string>('run_script', { path: s.path, lang: s.lang }); status.value = `✅ Выполнено` }
  catch (e) { status.value = '❌ ' + e }
}
</script>

<template>
  <div>
    <div class="header"><h1>📜 Script Hub</h1><button class="btn-primary" @click="openFolder">📂 Открыть папку</button></div>
    <div class="path-box"><code>{{ scriptsPath }}</code></div>
    <p v-if="status" class="status">{{ status }}</p>
    <div class="list">
      <div v-for="s in scripts" :key="s.path" class="script-item">
        <span class="lang-badge" :class="s.lang">{{ s.lang }}</span>
        <span>{{ s.name }}</span>
        <button class="btn-primary" @click="runScript(s)">▶ Запустить</button>
      </div>
    </div>
    <p v-if="!scripts.length" class="empty">Папка скриптов пуста</p>
  </div>
</template>

<style scoped>
.header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.path-box { background: var(--bg-card); padding: 15px; border-radius: 8px; margin-bottom: 20px; }
.path-box code { color: var(--accent); font-family: monospace; }
.list { display: flex; flex-direction: column; gap: 10px; }
.script-item { background: var(--bg-card); padding: 15px; border-radius: 8px; display: flex; align-items: center; gap: 12px; }
.script-item button { margin-left: auto; }
.lang-badge { padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.lang-badge.cpp { background: rgba(0,89,156,.3); color: #4fc3f7; }
.lang-badge.rust { background: rgba(222,165,132,.3); color: #dea584; }
.lang-badge.python { background: rgba(55,118,171,.3); color: #ffd43b; }
.status { color: var(--accent); margin-bottom: 15px; }
.empty { color: var(--text-dim); }
</style>