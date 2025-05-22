use crate::{BotData, BotError, err_fmt};
pub mod servers;

// TODO: Move loading server data over here

pub fn save_data(data: &BotData) -> Result<(), BotError> {
	// Saving servers
	if let Ok(server_read) = data.servers.read() {
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
					return Err(err_fmt!("Saving error (YML): `{err}`"));
				}
			}
		}
	} else {
		return Err(BotError::Str("Saving error: Failed unlocking `servers` lock"));
	}
	Ok(())
}
