use crate::{Context, BotError};
use crate::error::BotErrorExt;

/// Some information about teh bot!
#[poise::command(slash_command)]
pub async fn bot_info(ctx: Context<'_>) -> Result<(), BotError> {
    ctx.reply("Silly bot made by FlooferLand!\nRepo: <https://github.com/FlooferLand/tiny_discord_bot>").await.bot_err()?;
    Ok(())
}
