<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { login, register } from '../services/auth'

const router = useRouter()
const isLogin = ref(true)
const error = ref('')
const username = ref('')
const password = ref('')
const passwordConfirm = ref('')
const agreeTerms = ref(false)
const verificationCode = ref('')

async function submit() {
  error.value = ''
  try {
    if (isLogin.value) {
      await login(username.value, password.value)
    } else {
      await register({ username: username.value, password: password.value, passwordConfirm: passwordConfirm.value, agreeTerms: agreeTerms.value, verificationCode: verificationCode.value })
    }
    router.push('/games')
  } catch (e) { error.value = String(e) }
}
</script>

<template>
  <div class="auth-screen">
    <div class="auth-card">
      <div class="logo"><h1>XQC</h1><p>Game Launcher</p></div>
      <div class="tabs">
        <button :class="{ active: isLogin }" @click="isLogin = true">Вход</button>
        <button :class="{ active: !isLogin }" @click="isLogin = false">Регистрация</button>
      </div>
      <form @submit.prevent="submit" class="form">
        <input v-model="username" placeholder="Ник" required />
        <input v-model="password" type="password" placeholder="Пароль" required />
        <template v-if="!isLogin">
          <input v-model="passwordConfirm" type="password" placeholder="Повторите пароль" required />
          <label class="agree"><input type="checkbox" v-model="agreeTerms" /> Согласен с условиями</label>
          <input v-model="verificationCode" placeholder="Код подтверждения" required />
        </template>
        <button type="submit" class="btn-primary">{{ isLogin ? 'Войти' : 'Зарегистрироваться' }}</button>
        <p v-if="error" class="error">❌ {{ error }}</p>
      </form>
    </div>
  </div>
</template>

<style scoped>
.auth-screen { height: 100vh; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #1b2838, #171a21); }
.auth-card { background: rgba(42,63,90,.9); padding: 40px; border-radius: 12px; width: 420px; }
.logo { text-align: center; margin-bottom: 25px; }
.logo h1 { color: var(--accent); font-size: 52px; letter-spacing: 8px; }
.tabs { display: flex; border-bottom: 2px solid #3d5a80; margin-bottom: 20px; }
.tabs button { flex: 1; background: none; color: var(--text-dim); padding: 12px; font-size: 15px; }
.tabs button.active { color: var(--accent); border-bottom: 2px solid var(--accent); }
.form { display: flex; flex-direction: column; gap: 14px; }
.agree { display: flex; gap: 10px; align-items: center; color: var(--text-dim); font-size: 14px; }
.agree input { width: auto; }
.error { color: var(--red); font-size: 14px; }
</style>