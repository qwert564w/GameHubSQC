import { invoke } from '@tauri-apps/api/core'

export async function register(data: {
  username: string; password: string; passwordConfirm: string;
  agreeTerms: boolean; verificationCode: string
}): Promise<string> {
  return invoke('register', {
    username: data.username, password: data.password,
    passwordConfirm: data.passwordConfirm,
    agreeTerms: data.agreeTerms, verificationCode: data.verificationCode,
  })
}
export async function login(username: string, password: string): Promise<string> {
  return invoke('login', { username, password })
}
export async function logout(): Promise<void> { return invoke('logout') }
export async function checkAuth(): Promise<string | null> { return invoke('check_auth') }