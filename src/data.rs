use std::collections::HashMap;
use crate::{BotData, BotError, err_fmt};
pub(crate) use crate::data::servers::Server;

pub mod servers;

/// Panics if it can't load the data
#[allow(unused_parens)]
pub fn load_data() -> (HashMap<u64, Server>)  {
	// Loading server data
	let mut servers = HashMap::new();
	let _ = std::fs::create_dir_all("./assets/data/servers/");
	let servers_dir = std::fs::read_dir("./assets/data/servers/").unwrap();
	for entry in servers_dir {
		let Ok(dir) = entry else { continue };
		let Ok(server_text) = std::fs::read_to_string(dir.path()) else {
			panic!("Failed to parse server data");
		};
		let Ok(server_id) = dir.path().file_stem().unwrap_or(dir.file_name().as_os_str()).to_string_lossy().parse::<u64>() else {
			panic!("Failed to parse server ID");
		};
		match serde_yml::from_str(server_text.as_str()) {
			Ok(server_data) => {
				servers.insert(server_id, server_data);
			}
			Err(err) => {
				panic!("Failed to deserialize server data: {err}");
			}
		};
	}
	(servers)
}

pub async fn save_data(data: &BotData) -> Result<(), BotError> {
	// Saving servers
	let server_read = data.servers.read().await;
	for (key, server) in server_read.iter() {
		match serde_yml::to_string(&server) {
			Ok(out) => {
				let path = format!("./assets/data/servers/{key}.yml");
				let _ = std::fs::create_dir_all("./assets/data/servers/");
				if let Err(write_err) = std::fs::write(path, out) {
					return Err(err_fmt!("Saving error (DISK): `{write_err}`"));
				}
			}
			Err(err) => {
				return Err(err_fmt!("Saving error (SERDE): `{err}`"));
			}
		}
	}
	Ok(())
}
