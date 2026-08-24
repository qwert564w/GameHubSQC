use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoteFile { pub name: String, pub size: u64, pub download_url: Option<String> }
#[derive(Serialize, Deserialize, Clone)]
pub struct InstalledGame { pub name: String, pub exe_path: Option<String>, pub game_dir: Option<String> }
#[derive(Serialize, Deserialize, Clone)]
pub struct InstalledMod { pub name: String, pub file_name: String, pub local_path: Option<String> }
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateInfo { pub version: String, pub download_url: String }