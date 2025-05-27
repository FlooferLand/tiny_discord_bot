use crate::command_str;
use crate::commands::char_add::{CREATE_CHAR_ID, CREATE_CHAR_NAME};
use crate::commands::char_use::{SAYAS_ID, SAYAS_NAME};
use crate::error::{BotError, BotErrorExt, OkExt};
use crate::util::read_server;
use crate::Context;
use poise::CreateReply;

#[poise::command(slash_command, aliases("listchars"))]
pub async fn list_chars(ctx: Context<'_>, ) -> Result<(), BotError> {
	// Getting the characters
	let characters = read_server(ctx, |server| {
		server.characters.clone().ok()
	})?;

	// Building the message
	let mut message = String::from("## Characters\n-# (`id`, `name`, `avatar_url`)\n\n");
	for (id, char) in &characters {
		message += &format!(
			"`{id}` \"{DisplayName}\" [`avatar_url`](<{AvatarUrl}>)\n",
			DisplayName = &char.display_name,
			AvatarUrl = &char.avatar_url
		);
	}
	if characters.is_empty() {
		message += &format!("\nNo characters found.\nUse the {} command to create a new character!", command_str!(CREATE_CHAR_ID, CREATE_CHAR_NAME));
	}
	message += &format!("\n-# **HINT:** Use the {} command to make a character say something!", command_str!(SAYAS_ID, SAYAS_NAME));

	// Sending the message
	ctx.send(CreateReply::default().ephemeral(true).content(message)).await.bot_err()?;
	Ok(())
}