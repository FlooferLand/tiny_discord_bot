use crate::ArcLock;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default, Debug)]
pub struct Server {
    pub id: u64,
    pub characters: DashMap<String, ServerCharacter>,
    pub channels: ArcLock<ServerChannels>,
    pub roles: ArcLock<ServerRoles>,
    pub config: ArcLock<ServerConfig>,
}
impl Server {
    pub(crate) fn from_serde(serde: SerdeServer) -> Self {
        Self {
            id: serde.id,
            characters: serde.characters.clone(),
            channels: ArcLock::new(RwLock::from(serde.channels.clone())),
            roles: ArcLock::new(RwLock::from(serde.roles.clone())),
            config: ArcLock::new(RwLock::from(serde.config.clone()))
        }
    }
    pub(crate) async fn to_serde(&self) -> SerdeServer {
        SerdeServer {
            id: self.id,
            characters: self.characters.clone(),
            channels: self.channels.read().await.clone(),
            roles: self.roles.read().await.clone(),
            config: self.config.read().await.clone()
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SerdeServer {
    #[serde(skip)] pub id: u64,
    #[serde(default)] pub characters: DashMap<String, ServerCharacter>,
    #[serde(default)] pub channels: ServerChannels,
    #[serde(default)] pub roles: ServerRoles,
    #[serde(default)] pub config: ServerConfig,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ServerCharacter {
    pub display_name: String,
    pub avatar_url: String,
    pub hooks: HashMap<u64, String>
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ServerRoles {
    pub moderator: Option<u64>
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ServerChannels {
    pub bots: Option<u64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub silly_messages: bool,
    pub allow_user_characters: bool
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            silly_messages: true,
            allow_user_characters: false
        }
    }
}
