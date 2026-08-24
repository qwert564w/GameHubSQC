<script setup lang="ts">
defineProps<{ name: string; installed: boolean; hasPath: boolean }>()
defineEmits(['install', 'pick-path', 'play', 'folder'])
</script>

<template>
  <div class="card">
    <div class="banner">🎮</div>
    <div class="info">
      <h3>{{ name }}</h3>
      <button v-if="!installed" class="btn-primary" @click="$emit('install')">⬇ Установить</button>
      <button v-else-if="!hasPath" class="btn-path" @click="$emit('pick-path')">📂 Указать .exe</button>
      <div v-else class="actions">
        <button class="btn-play" @click="$emit('play')">▶ Играть</button>
        <button class="btn-folder" @click="$emit('folder')">📁</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card { background: var(--bg-card); border-radius: 8px; overflow: hidden; transition: all .3s; }
.card:hover { transform: translateY(-5px); box-shadow: 0 12px 32px rgba(0,0,0,.5); }
.banner { height: 120px; background: linear-gradient(135deg, #3d5a80, #1b2838); display: flex; align-items: center; justify-content: center; font-size: 52px; }
.info { padding: 15px; }
.info h3 { margin-bottom: 12px; }
.actions { display: flex; gap: 8px; }
.btn-play { background: var(--green); color: white; padding: 10px; flex: 1; }
.btn-folder { background: #3d5a80; color: white; padding: 10px; }
.btn-path { background: #d4a017; color: #171a21; padding: 10px; width: 100%; }
</style>