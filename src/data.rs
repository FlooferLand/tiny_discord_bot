use crate::{BotData, Error};
use anyhow::anyhow;

pub mod servers;

// TODO: Move loading server data over here

pub fn save_data(data: &BotData) -> Result<(), Error> {
	// Saving servers
	if let Ok(server_read) = data.servers.read() {
		for (key, server) in server_read.iter() {
			match serde_yml::to_string(&server) {
				Ok(out) => {
					let path = format!("./assets/data/servers/{key}.yml");
					if let Err(write_err) = std::fs::write(path, out) {
						return Err(anyhow!("Saving error (DISK): `{write_err}`"));
					}
				}
				Err(err) => {
					return Err(anyhow!("Saving error (YML): `{err}`"));
				}
			}
		}
	} else {
		return Err(anyhow!("Saving error: Failed unlocking `servers` lock"));
	}
	Ok(())
}
