use crate::commands::char::char_add::char_add;
use crate::commands::char::char_use::say_as;
use crate::error::{BotError, BotErrorExt};
use crate::{read_server, Context};
use poise::CreateReply;

/// List all the characters and info about them (IDs, display names, etc)
#[poise::command(slash_command, rename="list")]
pub(super) async fn char_list(ctx: Context<'_>) -> Result<(), BotError> {
	// Getting the characters
	let characters = read_server!(ctx, characters => { characters.clone() });

	// Building the message
	let mut message = String::from("## Characters\n-# (`id`, `name`, `avatar_url`)\n\n");
	for character in &characters {
		let (id, char) = (character.key(), character.value());
		message += &format!(
			"`{id}` \"{DisplayName}\" [`avatar_url`](<{AvatarUrl}>)\n",
			DisplayName = &char.display_name,
			AvatarUrl = &char.avatar_url
		);
	}
	if characters.is_empty() {
		message += &format!("\nNo characters found.\nUse the {} command to create a new character!", char_add().name);
	}
	message += &format!("\n-# **HINT:** Use the {} command to make a character say something!", say_as().name);

	// Sending the message
	ctx.send(CreateReply::default().ephemeral(true).content(message)).await.bot_err()?;
	Ok(())
}