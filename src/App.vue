<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import InstallerView from './views/InstallerView.vue'

const mode = ref<'installer' | 'launcher' | 'loading'>('loading')

onMounted(async () => {
  const installed = await invoke<boolean>('installer_get_status').catch(() => true)
  mode.value = installed ? 'launcher' : 'installer'
})
</script>

<template>
  <div v-if="mode === 'loading'" class="loading"><div class="spinner"></div></div>
  <InstallerView v-else-if="mode === 'installer'" />
  <router-view v-else />
</template>

<style scoped>
.loading { height: 100vh; display: flex; align-items: center; justify-content: center; background: #171a21; }
.spinner { width: 40px; height: 40px; border: 3px solid rgba(102,192,244,.2); border-top-color: #66c0f4; border-radius: 50%; animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>