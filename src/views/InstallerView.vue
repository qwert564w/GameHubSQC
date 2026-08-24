<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const percent = ref(0)
const message = ref('Нажмите "Установить"')
const done = ref(false)
const error = ref('')
const started = ref(false)

onMounted(async () => {
  await listen<{ percent: number; message: string; done: boolean; error: string | null }>('install-progress', (e) => {
    const p = e.payload
    if (p.error) { error.value = p.error; return }
    percent.value = p.percent; message.value = p.message
    if (p.done) done.value = true
  })
  const installed = await invoke<boolean>('installer_get_status')
  if (installed) { done.value = true; started.value = true; percent.value = 100 }
})

async function startInstall() { started.value = true; error.value = ''; await invoke('installer_start') }
function copyCommand() { navigator.clipboard.writeText('xqc run').catch(() => {}) }
function finish() { invoke('exit_app') }
</script>

<template>
  <div class="installer">
    <div class="logo"><h1>XQC</h1><p>Setup</p></div>
    <div v-if="!started" class="welcome">
      <button class="btn-install" @click="startInstall">Установить</button>
    </div>
    <div v-else-if="!done" class="progress-screen">
      <div class="progress-bar"><div class="progress-fill" :style="{ width: percent + '%' }"></div></div>
      <div class="progress-info"><span>{{ percent }}%</span><span>{{ message }}</span></div>
    </div>
    <div v-else class="done-screen">
      <div v-if="error" class="error-box"><p>{{ error }}</p><button class="btn-install" @click="startInstall">Повторить</button></div>
      <div v-else>
        <h2>✅ Готово!</h2>
        <div class="command-box">&gt; xqc run <button @click="copyCommand">📋</button></div>
        <button class="btn-install" @click="finish">Закрыть</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.installer { height: 100vh; display: flex; flex-direction: column; align-items: center; justify-content: center; background: linear-gradient(160deg, #171a21, #1b2838); padding: 30px; text-align: center; }
.logo h1 { font-size: 54px; color: #66c0f4; letter-spacing: 10px; }
.logo p { color: #8f98a0; }
.welcome { margin-top: 30px; }
.btn-install { background: #66c0f4; color: #171a21; border: none; padding: 13px 40px; border-radius: 6px; font-size: 16px; font-weight: bold; cursor: pointer; }
.progress-screen { width: 100%; max-width: 400px; margin-top: 30px; }
.progress-bar { width: 100%; height: 28px; background: #1a2634; border-radius: 14px; overflow: hidden; }
.progress-fill { height: 100%; background: linear-gradient(90deg, #2d7a3a, #5ba32b, #7ec850); border-radius: 14px; transition: width .4s; }
.progress-info { display: flex; justify-content: space-between; margin-top: 12px; }
.command-box { background: #0d1117; border: 1px solid #2a3f5a; border-radius: 8px; padding: 15px; font-family: monospace; color: #7ec850; margin: 15px 0; }
.error-box { background: rgba(255,107,107,.1); border: 1px solid #ff6b6b; padding: 20px; border-radius: 8px; }
</style>