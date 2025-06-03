use crate::Context;
use poise::CreateReply;
pub(crate) use crate::data::save_data;

/// Discord has an annoying need for apps to send a reply message to users.
/// Calling this function at the end of a command removes that
pub async fn consume_interaction<'a>(ctx: Context<'a>) {
	let _ = ctx.defer_ephemeral().await;
	let message = ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await;
	if let Ok(message) = message {
		message.delete(ctx).await.unwrap()
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __init_server_if_doesnt_exist {
    ($ctx:expr) => {{
	    let guild_id = $ctx.guild_id().bot_err("No guild ID found")?.get();
	    let server_exists = $ctx.data().servers.read().await.contains_key(&guild_id);

		// Initializing if it doesn't exist
		if !server_exists {
			let mut servers_write = $ctx.data().servers.write().await;
			match servers_write.get_mut(&guild_id) {
				None => {
					// Initializing a server if it doesn't exist
					servers_write.insert(guild_id, crate::data::Server::default());
					servers_write.get_mut(&guild_id).bot_err("Unable to initialize new server")?
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
	    crate::read_server_inner!($ctx, server => {
		    let $server_data = server.$server_data.read().await;
			$reader
	    })
    }};
}

/// Read directly from a server's data.
#[macro_export]
macro_rules! read_server_inner {
    ($ctx:expr, $server:ident => $reader:block) => {{
	    use crate::error::BotErrorMsgExt;
		let guild_id = $ctx.guild_id().bot_err("No guild ID found")?.get();

		// TODO: Find a way to reuse the read lock from this
		crate::__init_server_if_doesnt_exist!($ctx);

		let servers_read = $ctx.data().servers.read().await;
		let $server = servers_read.get(&guild_id).bot_err("Unable to find server, despite already having tried to initialize the server")?;
		$reader
    }};
}

/// Write something to a server's data.
/// Automatically writes the new data to disk.
#[macro_export]
macro_rules! write_server {
    ($ctx:expr, $server_data:ident => $writer:block) => {{
	    crate::write_server_inner!($ctx, server => {
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
	    use crate::error::BotErrorMsgExt;
        let guild_id = $ctx.guild_id().bot_err("No guild ID found")?;
		let out = {
			let mut servers_write = $ctx.data().servers.write().await;
			let $server = match servers_write.get_mut(&guild_id.get()) {
				None => {
					// Initializing a server if it doesn't exist
					servers_write.insert(guild_id.get(), crate::data::Server::default());
					servers_write
						.get_mut(&guild_id.get())
						.bot_err("Unable to initialize new server")?
				}
				Some(value) => value
			};
			$writer
		};
		crate::util::save_data($ctx.data()).await?;
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
