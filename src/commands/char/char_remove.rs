use crate::autocomplete::character;
use crate::error::{BotError, BotErrorExt};
use poise::CreateReply;
use crate::{send_message, write_server, Context};

/// Remove a character (warning: there is no removal confirmation)
#[poise::command(slash_command, rename="remove")]
pub(super) async fn char_remove(
	ctx: Context<'_>,
	#[autocomplete="character"] id: String
) -> Result<(), BotError> {
	write_server!(ctx, characters => {
		characters.remove(&id).bot_err("Character does not exist!")?
	});

	send_message!(
		ctx,
		CreateReply::default()
			.content(format!("Removed `{id}`!"))
			.ephemeral(true)
	)?;
	Ok(())
}