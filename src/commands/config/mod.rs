use poise::CreateReply;
use crate::Context;
use crate::error::{BotError, BotErrorExt};

mod config_set;
mod config_get;
use config_set::config_set;
use config_get::config_get;

#[poise::command(
	slash_command,
	rename="config",
	subcommands(
		"config_set",
		"config_get"
	)
)]
pub async fn config(ctx: Context<'_>, ) -> Result<(), BotError> {
	ctx.send(CreateReply::default().content("Use a subcommand!").ephemeral(true)).await.bot_err()?;
	Ok(())
}