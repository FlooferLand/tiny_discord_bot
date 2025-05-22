use std::collections::HashMap;
use crate::data::save_data;
use crate::{Context, BotError};
use poise::CreateReply;
use poise::serenity_prelude::json::{json, Value};
use crate::data::servers::ServerCharacter;
use crate::error::{BotErrorMsgExt, BotErrorExt};

#[poise::command(slash_command, aliases("createchar"))]
pub async fn create_char(
	ctx: Context<'_>,
	#[description = "You will use this to refer to the character. Use the `snake_case` convention"] id: String,
	#[description = "The name that will be shown to users"] display_name: String,
	#[description = "URL for the avatar; Must end in png, webp, etc"] avatar_url: String
) -> Result<(), BotError> {
	let guild_id = ctx.guild_id().bot_err("No guild ID found")?;

	let mut init = json!({ "name": display_name, "avatar_url": avatar_url });
	let map = init.as_object_mut().bot_err("Failed making webhook")?;
	let webhook = ctx.http().create_webhook(
		ctx.channel_id(),
		&Value::Object(map.clone()),
		None
	).await.bot_err()?;

	if let Ok(mut servers_write) = ctx.data().servers.write() {
		let server = servers_write.get_mut(&guild_id.get()).bot_err("Unknown server")?;
		if server.characters.contains_key(&id) {
			return Err(BotError::Str("Character already exists!"))
		}
		
		let mut hooks = HashMap::new();
		hooks.insert(ctx.channel_id().get(), webhook.url().bot_err()?);
		server.characters.insert(id, ServerCharacter { display_name, avatar_url, hooks });
	}

	ctx.send(CreateReply::default().content("-# Saved!").ephemeral(true)).await.bot_err()?;
	save_data(ctx.data())
}
