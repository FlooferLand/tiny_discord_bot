use poise::CreateReply;
use crate::Context;
use crate::error::{BotError, BotErrorExt};

pub mod char_use;
pub mod char_add;
pub mod char_list;
pub mod char_remove;

use char_use::char_use;
use char_add::char_add;
use char_list::char_list;
use char_remove::char_remove;

#[poise::command(
	slash_command,
	rename="char",
	subcommands(
		"char_add",
		"char_use",
		"char_list",
		"char_remove"
	),
	category = "Character",
	subcommand_required
)]
pub async fn char(ctx: Context<'_>, ) -> Result<(), BotError> {
	ctx.send(CreateReply::default().content("Use a subcommand!").ephemeral(true)).await.bot_err()?;
	Ok(())
}