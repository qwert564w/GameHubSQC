use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HotkeyConfig { pub key: String, pub enabled: bool }