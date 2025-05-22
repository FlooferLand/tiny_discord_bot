use crate::{Context, BotError};
use crate::error::BotErrorExt;

#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), BotError> {
    ctx.reply("Silly bot made by FlooferLand!\nRepo: <https://github.com/FlooferLand/tiny_discord_bot>").await.bot_err()?;
    Ok(())
}
