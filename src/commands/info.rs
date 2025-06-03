use crate::error::BotErrorExt;
use crate::{BotError, Context};
use indoc::formatdoc;

/// Some information about teh bot!
#[poise::command(slash_command, rename="info")]
pub async fn bot_info(ctx: Context<'_>) -> Result<(), BotError> {
    let message = formatdoc!(
        r#"
            Silly bot made by FlooferLand!
            Repo: <https://github.com/FlooferLand/tiny_discord_bot>
            Version: `{Version}`
        "#,
        Version = env!("CARGO_PKG_VERSION")
    );
    ctx.reply(message).await.bot_err()?;
    Ok(())
}
