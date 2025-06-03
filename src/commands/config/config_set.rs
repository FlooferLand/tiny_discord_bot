use crate::Context;
use crate::error::BotError;

/// Set a value inside the bot's config
#[poise::command(slash_command, rename="set")]
pub(super) async fn config_set(
	ctx: Context<'_>,
	id: String,
	value: String
) -> Result<(), BotError> {
	Ok(())
}