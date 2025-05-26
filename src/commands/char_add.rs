use crate::data::save_data;
use crate::data::servers::{Server, ServerCharacter};
use crate::error::BotErrorMsgExt;
use crate::util::swallow_interaction;
use crate::{err_fmt, BotError, Context};
use std::collections::HashMap;
use poise::serenity_prelude::UserId;

#[poise::command(slash_command, aliases("createchar"))]
pub async fn create_char(
	ctx: Context<'_>,
	#[description = "You will use this to refer to the character. Use the `snake_case` convention"] id: String,
	#[description = "The name that will be shown to users"] display_name: String,
	#[description = "URL for the avatar; Must end in png, webp, etc"] avatar_url: String
) -> Result<(), BotError> {
	let guild_id = ctx.guild_id().bot_err("No guild ID found")?;

	// Checking things
	let id = id.to_ascii_lowercase().replace(' ', "_");
	let avatar_url = if avatar_url.starts_with("http") {
		avatar_url
	} else {
		if let Ok(id) = avatar_url.parse::<u64>() {
			if let Ok(user) = ctx.http().get_user(UserId::new(id)).await {
				match user.avatar_url() {
					Some(avatar_url) => avatar_url,
					None => user.default_avatar_url()
				}
			} else {
				return Err(err_fmt!("It says '{}' doofus, input a valid URL!\nI secretly do possess the ability of parsing user IDs, but you didn't even give me a valid ID!", stringify!(avatar_url)));
			}
		} else {
			return Err(err_fmt!("It says '{}' doofus, input a valid URL!", stringify!(avatar_url)));
		}
	};

	// Writing the new character
	if let Ok(mut servers_write) = ctx.data().servers.write() {
		let server = match servers_write.get_mut(&guild_id.get()) {
			None => {
				// Initializing a server if it doesn't exist
				servers_write.insert(guild_id.get(), Server::default());
				servers_write.get_mut(&guild_id.get()).bot_err("Unable to initialize new server")?
			}
			Some(value) => value
		};
		if server.characters.contains_key(&id) {
			return Err(BotError::Str("Character already exists!"))
		}
		
		let hooks = HashMap::new();
		server.characters.insert(id, ServerCharacter { display_name, avatar_url, hooks });
	}

	swallow_interaction(ctx).await;
	save_data(ctx.data())
}
