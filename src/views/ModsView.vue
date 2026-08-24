<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ModCard from '../components/ModCard.vue'
import { fetchRemoteMods, getInstalledMods, installMod } from '../services/github'
import type { RemoteFile, InstalledMod } from '../services/github'

const remoteMods = ref<RemoteFile[]>([])
const installed = ref<InstalledMod[]>([])
const status = ref('')

onMounted(refresh)

async function refresh() {
  remoteMods.value = (await fetchRemoteMods().catch(() => [])).filter(m => m.name.endsWith('.jar'))
  installed.value = await getInstalledMods().catch(() => [])
}

const isInstalled = (name: string) => installed.value.some(m => m.file_name === name)

async function doInstall(mod: RemoteFile) {
  status.value = '⬇ Скачивание...'
  try {
    await installMod(mod.name, mod.download_url!)
    status.value = '✅ Мод установлен!'
    await refresh()
  } catch (e) { status.value = '❌ ' + e }
}
</script>

<template>
  <div>
    <h1>🛒 Магазин модов</h1>
    <p v-if="status" class="status">{{ status }}</p>
    <div class="list">
      <ModCard v-for="mod in remoteMods" :key="mod.name"
        :name="mod.name" :size="mod.size" :installed="isInstalled(mod.name)"
        @install="doInstall(mod)" />
    </div>
    <p v-if="!remoteMods.length" class="empty">Папка mod/ пуста</p>
  </div>
</template>

<style scoped>
h1 { margin-bottom: 20px; }
.list { display: flex; flex-direction: column; gap: 12px; }
.status { color: var(--accent); margin: 10px 0; }
.empty { color: var(--text-dim); }
</style>