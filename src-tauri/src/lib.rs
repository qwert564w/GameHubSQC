use tauri::Manager;

mod crypto;
mod github;
mod installer;
mod panic;

#[tauri::command]
fn register(username: String, password: String, password_confirm: String, agree_terms: bool, verification_code: String) -> Result<String, String> {
    Ok("ok".into())
}
#[tauri::command]
fn login(username: String, password: String) -> Result<String, String> { Ok("ok".into()) }
#[tauri::command]
fn logout() -> Result<(), String> { Ok(()) }
#[tauri::command]
fn check_auth() -> Result<Option<String>, String> { Ok(Some("user".into())) }

#[tauri::command]
fn fetch_remote_games() -> Result<Vec<github::RemoteFile>, String> { Ok(vec![]) }
#[tauri::command]
fn install_game(name: String, url: String) -> Result<String, String> { Ok("ok".into()) }
#[tauri::command]
fn get_installed_games() -> Result<Vec<github::InstalledGame>, String> { Ok(vec![]) }
#[tauri::command]
fn launch_game(path: String, name: String) -> Result<(), String> { Ok(()) }
#[tauri::command]
fn open_folder(path: String) -> Result<(), String> { Ok(()) }
#[tauri::command]
fn pick_game_exe() -> Result<Option<String>, String> { Ok(None) }
#[tauri::command]
fn set_game_path(name: String, path: String) -> Result<(), String> { Ok(()) }

#[tauri::command]
fn fetch_remote_mods() -> Result<Vec<github::RemoteFile>, String> { Ok(vec![]) }
#[tauri::command]
fn install_mod(name: String, url: String) -> Result<String, String> { Ok("ok".into()) }
#[tauri::command]
fn get_installed_mods() -> Result<Vec<github::InstalledMod>, String> { Ok(vec![]) }

#[tauri::command]
fn get_scripts_dir() -> Result<String, String> { Ok("".into()) }
#[tauri::command]
fn list_scripts() -> Result<Vec<installer::Script>, String> { Ok(vec![]) }
#[tauri::command]
fn open_scripts_folder() -> Result<(), String> { Ok(()) }
#[tauri::command]
fn run_script(path: String, lang: String) -> Result<String, String> { Ok("ok".into()) }

#[tauri::command]
fn check_update() -> Result<Option<github::UpdateInfo>, String> { Ok(None) }
#[tauri::command]
fn apply_update(url: String) -> Result<(), String> { Ok(()) }

#[tauri::command]
fn get_panic_hotkey() -> Result<panic::HotkeyConfig, String> { Ok(panic::HotkeyConfig { key: "".into(), enabled: false }) }
#[tauri::command]
fn set_panic_hotkey(key: String, enabled: bool) -> Result<(), String> { Ok(()) }
#[tauri::command]
fn panic_now() -> Result<Vec<String>, String> { Ok(vec![]) }
#[tauri::command]
fn panic_wipe() -> Result<(), String> { Ok(()) }

#[tauri::command]
fn installer_get_status() -> Result<bool, String> { Ok(false) }
#[tauri::command]
fn installer_start(app_handle: tauri::AppHandle) -> Result<(), String> { Ok(()) }
#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}
#[tauri::command]
fn generate_access_token() -> Result<String, String> { Ok("token".into()) }

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            register, login, logout, check_auth,
            fetch_remote_games, install_game, get_installed_games, launch_game, open_folder, pick_game_exe, set_game_path,
            fetch_remote_mods, install_mod, get_installed_mods,
            get_scripts_dir, list_scripts, open_scripts_folder, run_script,
            check_update, apply_update,
            get_panic_hotkey, set_panic_hotkey, panic_now, panic_wipe,
            installer_get_status, installer_start, exit_app, generate_access_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run_installer() {
    // placeholder
}