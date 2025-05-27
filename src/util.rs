use crate::Context;
use poise::CreateReply;
use crate::data::servers::Server;
use crate::error::{BotError, BotErrorExt, BotErrorMsgExt};

/// Discord has an annoying need for apps to send a reply message to users.
/// Calling this function at the end of a command removes that
pub async fn consume_interaction<'a>(ctx: Context<'a>) {
	let _ = ctx.defer_ephemeral().await;
	let message = ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await;
	if let Ok(message) = message {
		message.delete(ctx).await.unwrap()
	}
}

/// Read from a server's data
pub fn read_server<Func, Out>(ctx: Context, reader: Func) -> Result<Out, BotError>
where Out: 'static,
      Func: Fn(&Server) -> Result<Out, BotError> {
	let guild_id = ctx.guild_id().bot_err("No guild ID found")?;

	let _ = write_server(ctx, |_| { Ok(()) }); // Initializing if it doesn't exist

	let servers_read = ctx.data().servers.read().bot_err()?;
	let server = servers_read.get(&guild_id.get()).bot_err("Unable to find server, despite already having tried to initialize the server")?;
	reader(server)
}

/// Write something to a server's data
pub fn write_server<Func, Out>(ctx: Context, writer: Func) -> Result<Out, BotError>
where Func: FnOnce(&mut Server) -> Result<Out, BotError> {
	let guild_id = ctx.guild_id().bot_err("No guild ID found")?;

	let mut servers_write = ctx.data().servers.write().bot_err()?;
	let server = match servers_write.get_mut(&guild_id.get()) {
		None => {
			// Initializing a server if it doesn't exist
			servers_write.insert(guild_id.get(), Server::default());
			servers_write.get_mut(&guild_id.get()).bot_err("Unable to initialize new server")?
		}
		Some(value) => value
	};
	writer(server)
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
