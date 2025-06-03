use crate::serde::arc_rwlock_serde;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ArcLock;

#[derive(Serialize, Deserialize, Default)]
pub struct Server {
    #[serde(default)]
    #[serde(with = "arc_rwlock_serde")]
    pub characters: ArcLock<HashMap<String, ServerCharacter>>,

    #[serde(default)]
    #[serde(with = "arc_rwlock_serde")]
    pub roles: ArcLock<ServerRoles>,

    #[serde(default)]
    #[serde(with = "arc_rwlock_serde")]
    pub config: ArcLock<ServerConfig>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ServerCharacter {
    pub display_name: String,
    pub avatar_url: String,
    pub hooks: HashMap<u64, String>
}

#[derive(Serialize, Deserialize, Default)]
pub struct ServerRoles {
    pub moderator: Option<u64>
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub silly_messages: bool
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            silly_messages: false
        }
    }
}
