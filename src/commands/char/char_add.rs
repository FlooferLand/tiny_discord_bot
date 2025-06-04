use crate::data::servers::ServerCharacter;
use crate::util::consume_interaction;
use crate::{err_fmt, write_server, BotError, Context};
use poise::serenity_prelude::UserId;
use std::collections::HashMap;
use indoc::formatdoc;

const VALID_IMAGE_TYPES: [&str; 4] = ["webp", "png", "jpg", "jpeg"];

/// Add a new character
#[poise::command(slash_command, rename="add")]
pub(super) async fn char_add(
	ctx: Context<'_>,
	#[description = "You will use this to refer to the character. Use the `snake_case` convention"] id: String,
	#[description = "The name that will be shown to users"] display_name: String,
	#[description = "URL for the avatar; Must end in png, webp, etc"] avatar_url: String
) -> Result<(), BotError> {
	// Checking if the ID is valid
	let id = id.trim().to_ascii_lowercase().replace(' ', "_");
	if !id.is_ascii() {
		return Err(BotError::Str("The character ID must be ASCII.\nEx: `hawtsaus`, `chickn_nuggit`"))
	}

	// Checking if the avatar (MY CABBAGES) is valid
	let avatar_url = {
		let avatar_url = match avatar_url.split('?').collect::<Vec<&str>>().first() {
			Some(v) => v.to_string(),
			None => avatar_url
		};

		let is_valid_image = avatar_url.starts_with("http")
			&& VALID_IMAGE_TYPES.iter().any(|ext| avatar_url.ends_with(&format!(".{}", ext)));

		if is_valid_image {
			avatar_url
		} else if let Ok(id) = avatar_url.parse::<u64>() {
            if let Ok(user) = ctx.http().get_user(UserId::new(id)).await {
                match user.avatar_url() {
                    Some(avatar_url) => avatar_url,
                    None => user.default_avatar_url()
                }
            } else {
                return Err(BotError::String(formatdoc! {"
                    It says '{url}' doofus, input a valid URL!
	                I secretly do possess the ability of parsing user IDs, but you didn't even give me a valid ID!
                    ",
	                url = stringify!(avatar_url)
                }));
            }
        } else {
            return Err(err_fmt!("It says '{}' doofus, input a valid URL!", stringify!(avatar_url)));
        }
	};

	// Writing the new character
	let new_char = ServerCharacter { display_name, avatar_url, hooks: HashMap::new() };
	write_server!(ctx, characters => {
		if characters.contains_key(&id) {
			return Err(BotError::Str("Character already exists!"))
		}
		characters.insert(id.clone(), new_char);
	});

	consume_interaction(ctx).await;
	Ok(())
}
