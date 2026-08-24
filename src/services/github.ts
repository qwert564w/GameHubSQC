import { invoke } from '@tauri-apps/api/core'

export interface RemoteFile { name: string; size: number; download_url: string | null }
export interface InstalledGame { name: string; exe_path: string | null; game_dir: string | null }
export interface InstalledMod { name: string; file_name: string; local_path: string | null }

export async function fetchRemoteGames(): Promise<RemoteFile[]> { return invoke('fetch_remote_games') }
export async function fetchRemoteMods(): Promise<RemoteFile[]> { return invoke('fetch_remote_mods') }
export async function installGame(name: string, url: string): Promise<string> { return invoke('install_game', { name, url }) }
export async function installMod(name: string, url: string): Promise<string> { return invoke('install_mod', { name, url }) }
export async function getInstalledGames(): Promise<InstalledGame[]> { return invoke('get_installed_games') }
export async function getInstalledMods(): Promise<InstalledMod[]> { return invoke('get_installed_mods') }
export async function launchGame(path: string, name: string): Promise<void> { return invoke('launch_game', { path, name }) }
export async function openFolder(path: string): Promise<void> { return invoke('open_folder', { path }) }
export async function pickGameExe(): Promise<string | null> { return invoke('pick_game_exe') }
export async function setGamePath(name: string, path: string): Promise<void> { return invoke('set_game_path', { name, path }) }
export async function checkUpdate(): Promise<{ version: string; download_url: string } | null> { return invoke('check_update') }