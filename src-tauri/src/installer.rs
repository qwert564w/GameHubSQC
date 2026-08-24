use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Script { pub name: String, pub path: String, pub lang: String }