use crate::Context;
use dashmap::DashMap;
use poise::CreateReply;

/// Discord has an annoying need for apps to send a reply message to users.
/// Calling this function at the end of a command removes that
pub async fn consume_interaction<'a>(ctx: Context<'a>) {
	let _ = ctx.defer_ephemeral().await;
	let message = ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await;
	if let Ok(message) = message {
		message.delete(ctx).await.unwrap()
	}
}

// Read/write macro util
pub trait DashMapReadWrite<'a, K: 'a, V: 'a> {
	async fn read(&'a self) -> &'a DashMap<K, V>;
	async fn write(&'a self) -> &'a DashMap<K, V>;
}
impl<'a, K: 'a, V: 'a> DashMapReadWrite<'a, K, V> for DashMap<K, V> {
	async fn read(&self) -> &DashMap<K, V> {
		self
	}

	async fn write(&self) -> &DashMap<K, V> {
		self
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __init_server_if_doesnt_exist {
    ($ctx:expr) => {{
	    let guild_id = $ctx.guild_id().bot_err("No guild ID found")?.get();
	    let servers = &$ctx.data().servers;
	    let server_exists = servers.contains_key(&guild_id);

		// Initializing if it doesn't exist
		if !server_exists {
			match servers.get_mut(&guild_id) {
				None => {
					// Initializing a server if it doesn't exist
					servers.insert(guild_id, crate::data::Server::default());
					servers.get_mut(&guild_id).bot_err("Unable to initialize new server")?
				}
				Some(value) => value
			};
		}
    }};
}

/// Read from a server's data.
#[macro_export]
macro_rules! read_server {
    ($ctx:expr, $server_data:ident => $reader:block) => {{
	    use $crate::util::DashMapReadWrite;
	    $crate::read_server_inner!($ctx, server => {
		    let $server_data = server.$server_data.read().await;
			$reader
	    })
    }};
}

/// Read directly from a server's data.
#[macro_export]
macro_rules! read_server_inner {
    ($ctx:expr, $server:ident => $reader:block) => {{
	    use $crate::error::BotErrorMsgExt;
		let guild_id = $ctx.guild_id().bot_err("No guild ID found")?.get();

		// TODO: Find a way to reuse the read lock from this
		$crate::__init_server_if_doesnt_exist!($ctx);

		let servers = &$ctx.data().servers;
		let $server = servers.get(&guild_id).bot_err("Unable to find server, despite already having tried to initialize the server")?;
		$reader
    }};
}

/// Write something to a server's data.
/// Automatically writes the new data to disk.
#[macro_export]
macro_rules! write_server {
    ($ctx:expr, $server_data:ident => $writer:block) => {{
	    use $crate::util::DashMapReadWrite;
	    $crate::write_server_inner!($ctx, server => {
		    #[allow(unused_mut)]
			let mut $server_data = server.$server_data.write().await;
		    $writer
	    })
    }};
}
/// Write something directly to a server's data.
/// Automatically writes the new data to disk.
#[macro_export]
macro_rules! write_server_inner {
    ($ctx:expr, $server:ident => $writer:block) => {{
	    use $crate::error::BotErrorMsgExt;
        let guild_id = $ctx.guild_id().bot_err("No guild ID found")?;
		let out = {
			let servers = &$ctx.data().servers;
			let $server = match servers.get_mut(&guild_id.get()) {
				None => {
					// Initializing a server if it doesn't exist
					servers.insert(guild_id.get(), $crate::data::Server::default());
					servers
						.get_mut(&guild_id.get())
						.bot_err("Unable to initialize new server")?
				}
				Some(value) => value
			};
			$writer
		};
		$crate::data::save_data($ctx).await?;
		out
    }};
}

// String manipulation
#[macro_export]
macro_rules! command_str {
    ($id:ident, $name:ident) => {{
	    const NAME: &str = $name;
	    const ID: u64 = $id;
	    const_format::concatcp!("</", NAME, ":" , ID, ">")
    }};
}
