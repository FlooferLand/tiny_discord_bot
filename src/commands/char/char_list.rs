use crate::commands::char::char_add::char_add;
use crate::commands::char::char_use::say_as;
use crate::error::{BotError, BotErrorExt};
use crate::{command_str, read_server, send_message, Context};
use poise::CreateReply;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use crate::commands::char::char;

/// List all the characters and info about them (IDs, display names, etc)
#[poise::command(slash_command, rename="list")]
pub(super) async fn char_list(ctx: Context<'_>) -> Result<(), BotError> {
	// Getting the characters
	let characters = read_server!(ctx, characters => { characters.clone() });

	// Building the reply
	let mut reply = CreateReply::default().ephemeral(true);

	// Building the message
	for character in &characters {
		let (id, char) = (character.key(), character.value());

		let embed = CreateEmbed::new()
			.title(&char.display_name)
			.field("ID", id, true)
			.thumbnail(&char.avatar_url);
		reply.embeds.push(embed);
	}
	if characters.is_empty() {
		let embed = CreateEmbed::new()
			.title("No characters found")
			.description(format!("Use the `{}` command to create a new character!", command_str!(char(), char_add())));
		reply.embeds.push(embed);
	} else {
		reply.content = Some(format!("\n**HINT:** Use a character's ID in the `{}` command to make them say something!", command_str!(say_as())));
	}

	// Sending the message
	send_message!(ctx, reply)?;
	Ok(())
}