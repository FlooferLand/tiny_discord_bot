use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Server {
    pub roles: ServerRoles,
    pub characters: HashMap<String, ServerCharacter>
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
