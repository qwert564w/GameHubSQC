<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const hotkey = ref('')
const enabled = ref(true)
const recording = ref(false)
const pressedKeys = ref<string[]>([])
const status = ref('')

onMounted(async () => {
  const hk = await invoke<{ key: string; enabled: boolean }>('get_panic_hotkey')
  hotkey.value = hk.key; enabled.value = hk.enabled
})

const KEYS_MAP: Record<string, string> = { ControlLeft: 'Ctrl', ControlRight: 'Ctrl', ShiftLeft: 'Shift', ShiftRight: 'Shift', AltLeft: 'Alt', AltRight: 'Alt' }

function startRecording() { recording.value = true; pressedKeys.value = []; window.addEventListener('keydown', onKey); status.value = 'Нажми комбинацию...' }
function stopRecording() { recording.value = false; window.removeEventListener('keydown', onKey); if (pressedKeys.value.length) hotkey.value = pressedKeys.value.join('+'); status.value = '' }
function onKey(e: KeyboardEvent) { if (!recording.value) return; e.preventDefault(); const k = KEYS_MAP[e.code] || e.key.toUpperCase(); if (!pressedKeys.value.includes(k)) pressedKeys.value.push(k); if (pressedKeys.value.length >= 2) stopRecording() }

async function save() { await invoke('set_panic_hotkey', { key: hotkey.value, enabled: enabled.value }); status.value = '✅ Сохранено!' }
async function testPanic() { if (!confirm('Закрыть все игры?')) return; const k = await invoke<string[]>('panic_now'); status.value = k.length ? `Закрыто: ${k.join(', ')}` : 'Нет игр' }
async function wipe() { if (!confirm('УДАЛИТЬ ВСЕ ДАННЫЕ?')) return; await invoke('panic_wipe').catch(() => {}); status.value = '🗑️ Удалено' }
</script>

<template>
  <div>
    <h1>⚙️ Настройки</h1>
    <div class="section">
      <h2>🚨 Экстренное закрытие</h2>
      <label><input type="checkbox" v-model="enabled" /> Включить</label>
      <div class="hotkey-box">
        <span>{{ recording ? pressedKeys.join('+') || 'жми...' : hotkey || 'Не назначена' }}</span>
        <button v-if="recording" @click="stopRecording">Отмена</button>
        <button v-else class="btn-primary" @click="startRecording">Назначить</button>
      </div>
      <button class="btn-primary" @click="save">💾 Сохранить</button>
      <button class="btn-danger" @click="testPanic">🧪 Тест</button>
    </div>
    <div class="section danger">
      <h2>☠️ Зачистка</h2>
      <button class="btn-danger" @click="wipe">🗑️ Удалить все данные</button>
    </div>
    <p v-if="status" class="status">{{ status }}</p>
  </div>
</template>

<style scoped>
h1 { margin-bottom: 20px; }
.section { background: var(--bg-card); padding: 25px; border-radius: 8px; margin-bottom: 20px; }
.danger { border: 1px solid rgba(255,107,107,.3); }
h2 { color: var(--accent); margin-bottom: 15px; }
label input { margin-right: 8px; width: auto; }
.hotkey-box { display: flex; align-items: center; gap: 12px; background: var(--bg-dark); padding: 12px; border-radius: 6px; margin: 12px 0; }
button { margin: 5px 5px 0 0; }
.btn-danger { background: rgba(255,107,107,.15); color: #ff6b6b; border: 1px solid #ff6b6b; padding: 10px 20px; }
.btn-danger:hover { background: #ff6b6b; color: white; }
.status { margin-top: 15px; color: var(--accent); }
</style>