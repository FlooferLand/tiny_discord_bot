use crate::error::{BotErrorExt, BotErrorMsgExt};
use crate::util::write_server;
use poise::CreateReply;

/// Remove a character (warning: there is no removal confirmation)
#[poise::command(slash_command, rename="remove")]
pub(super) async fn char_remove(
	ctx: crate::Context<'_>,
	id: String
) -> Result<(), crate::error::BotError> {
	write_server(ctx, |server| {
		server.characters.remove(&id).bot_err("Character does not exist!")
	})?;

	ctx.send(
		CreateReply::default()
			.content(&format!("Removed `{id}`!"))
			.ephemeral(true)
	).await.bot_err()?;
	Ok(())
}