use crate::Context;
use crate::error::BotError;

/// Get a value from the bot's config
#[poise::command(slash_command, rename="get")]
pub(super) async fn config_get(
	ctx: Context<'_>,
	id: String
) -> Result<(), BotError> {
	Ok(())
}