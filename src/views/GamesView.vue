<script setup lang="ts">
import { ref, onMounted } from 'vue'
import GameCard from '../components/GameCard.vue'
import { fetchRemoteGames, getInstalledGames, installGame, launchGame, openFolder, pickGameExe, setGamePath } from '../services/github'
import type { RemoteFile, InstalledGame } from '../services/github'

const remoteGames = ref<RemoteFile[]>([])
const installed = ref<InstalledGame[]>([])
const status = ref('')

onMounted(refresh)

async function refresh() {
  remoteGames.value = await fetchRemoteGames().catch(() => [])
  installed.value = await getInstalledGames().catch(() => [])
}

const cleanName = (f: string) => f.replace('.exe', '').replace(/_installer/gi, '')
const getInstalled = (name: string) => installed.value.find(g => g.name === name)

async function doInstall(game: RemoteFile) {
  status.value = '⬇ Скачивание...'
  try {
    await installGame(cleanName(game.name), game.download_url!)
    status.value = '✅ Установщик запущен!'
    await refresh()
  } catch (e) { status.value = '❌ ' + e }
}

async function doPickPath(name: string) {
  const path = await pickGameExe()
  if (path) { await setGamePath(name, path); await refresh(); status.value = '✅ Путь сохранён' }
}

async function doPlay(name: string) {
  const g = getInstalled(name)
  if (!g?.exe_path) { status.value = '⚠️ Укажи путь к игре'; return }
  await launchGame(g.exe_path, name).catch(e => status.value = '❌ ' + e)
}

async function doFolder(name: string) {
  const g = getInstalled(name)
  if (g?.game_dir) await openFolder(g.game_dir)
}
</script>

<template>
  <div>
    <h1>🎮 Мои игры</h1>
    <p v-if="status" class="status">{{ status }}</p>
    <div class="grid">
      <GameCard v-for="game in remoteGames" :key="game.name"
        :name="cleanName(game.name)"
        :installed="!!getInstalled(cleanName(game.name))"
        :has-path="!!getInstalled(cleanName(game.name))?.exe_path"
        @install="doInstall(game)" @pick-path="doPickPath(cleanName(game.name))"
        @play="doPlay(cleanName(game.name))" @folder="doFolder(cleanName(game.name))" />
    </div>
    <p v-if="!remoteGames.length" class="empty">Папка game/ пуста</p>
  </div>
</template>

<style scoped>
h1 { margin-bottom: 20px; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 20px; }
.status { color: var(--accent); margin: 10px 0; }
.empty { color: var(--text-dim); }
</style>