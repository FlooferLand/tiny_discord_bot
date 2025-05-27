use crate::data::save_data;
use crate::data::servers::ServerCharacter;
use crate::util::{consume_interaction, write_server};
use crate::{err_fmt, BotError, Context};
use poise::serenity_prelude::UserId;
use std::collections::HashMap;

pub const CREATE_CHAR_NAME: &str = "create_char";
pub const CREATE_CHAR_ID: u64 = 1376351248919429183;

#[poise::command(slash_command, aliases("createchar"))]
pub async fn create_char(
	ctx: Context<'_>,
	#[description = "You will use this to refer to the character. Use the `snake_case` convention"] id: String,
	#[description = "The name that will be shown to users"] display_name: String,
	#[description = "URL for the avatar; Must end in png, webp, etc"] avatar_url: String
) -> Result<(), BotError> {
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
	let new_char = ServerCharacter { display_name, avatar_url, hooks: HashMap::new() };
	write_server(ctx, move |server| {
		if server.characters.contains_key(&id) {
			return Err(BotError::Str("Character already exists!"))
		}

		server.characters.insert(id.clone(), new_char);
		Ok(())
	})?;

	consume_interaction(ctx).await;
	save_data(ctx.data())
}
