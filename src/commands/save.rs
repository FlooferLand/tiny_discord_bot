use poise::CreateReply;
use crate::{Context, Error};
use crate::data::save_data;

#[poise::command(slash_command)]
pub async fn save(ctx: Context<'_>) -> Result<(), Error>  {
	ctx.send(CreateReply::default().content("-# Saved!").ephemeral(true)).await?;
	save_data(ctx.data())
}