use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Silly bot made by FlooferLand!\nRepo: <https://github.com/FlooferLand/tiny_discord_bot>").await?;
    Ok(())
}
