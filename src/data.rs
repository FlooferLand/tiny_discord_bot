use crate::data::servers::SerdeServer;
pub(crate) use crate::data::servers::Server;
use crate::{err_fmt, BotData, BotError, Context};
use dashmap::DashMap;
use indoc::formatdoc;
use poise::serenity_prelude::GuildId;

pub mod servers;

/// Panics if it can't load the data
#[allow(unused_parens)]
pub fn load_data() -> (DashMap<u64, Server>)  {
	// Loading server data
	let servers = DashMap::new();
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
		match serde_yml::from_str::<SerdeServer>(server_text.as_str()) {
			Ok(server_data) => {
				servers.insert(server_id, Server::from_serde(server_data));
			}
			Err(err) => {
				panic!("Failed to deserialize server data: {err}");
			}
		};
	}
	(servers)
}

pub async fn save_data<'a>(ctx: Context<'a>) -> Result<(), BotError> {
	// Saving servers
	for server in ctx.data().servers.iter() {
		let guild = ctx.http().get_guild(GuildId::new(*server.key())).await;
		match serde_yml::to_string(&server.to_serde().await) {
			Ok(out) => {
				// Fancy extra info
				let guild_name = guild
					.map(|guild| guild.name.clone())
					.unwrap_or("Unknown".to_string());
				let out = formatdoc!("
					# guild: {guild_name}
					{out}
				");
				
				// Writing the file
				let path = format!("./assets/data/servers/{}.yml", server.key());
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
