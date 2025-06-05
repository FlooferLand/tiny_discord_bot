use poise::CreateReply;
use crate::{Context, BotError};
use crate::data::save_data;
use crate::error::BotErrorExt;

#[poise::command(slash_command, owners_only)]
pub async fn save(ctx: Context<'_>) -> Result<(), BotError>  {
	ctx.send(CreateReply::default().content("-# Saved!").ephemeral(true)).await.bot_err()?;
	save_data(ctx).await
}