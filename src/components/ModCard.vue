<script setup lang="ts">
defineProps<{ name: string; size: number; installed: boolean }>()
defineEmits(['install'])
</script>

<template>
  <div class="mod-item">
    <div class="mod-info">
      <div class="mod-icon">📦</div>
      <div>
        <h3>{{ name.replace('.jar', '') }}</h3>
        <p>{{ (size / 1024).toFixed(1) }} KB</p>
      </div>
    </div>
    <span v-if="installed" class="badge">✅</span>
    <button v-else class="btn-primary" @click="$emit('install')">⬇ Установить</button>
  </div>
</template>

<style scoped>
.mod-item { background: var(--bg-card); padding: 18px; border-radius: 8px; display: flex; justify-content: space-between; align-items: center; }
.mod-info { display: flex; gap: 15px; align-items: center; }
.mod-icon { width: 50px; height: 50px; background: linear-gradient(135deg,#3d5a80,#1b2838); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px; }
.mod-info p { color: var(--text-dim); font-size: 13px; }
.badge { color: var(--green); font-size: 20px; }
</style>