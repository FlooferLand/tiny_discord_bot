use crate::error::{BotErrorMsgExt, OkExt};
use crate::fake_user::{FakeUserError, FakeUserMaker, WebhookMessage};
use crate::util::{consume_interaction, read_server};
use crate::{BotError, Context};

/// Use a character to send a message (shorthand for `char_use`)
#[poise::command(slash_command, rename="sayas")]
pub async fn say_as(
    ctx: Context<'_>,
    #[description = "ID"] id: String,
    #[description = "Content"] content: String
) -> Result<(), BotError> {
    Box::pin(inner(ctx, id, content)).await
}

/// Use a character to send a message
#[poise::command(slash_command, rename="use")]
pub(super) async fn char_use(
    ctx: Context<'_>,
    #[description = "ID"] id: String,
    #[description = "Content"] content: String
) -> Result<(), BotError> {
    Box::pin(inner(ctx, id, content)).await
}

#[allow(dead_code)]
async fn inner(ctx: Context<'_>, id: String, content: String) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().bot_err("No guild ID found")?;
    let id = id.trim();

    // Getting the character
    let char = read_server(ctx, |server| {
        server.characters.get(id).bot_err("Unable to find character")?.clone().ok()
    })?;

    // If the hook already exists..
    let mut has_existing_hook = true;
    if let Some(hook_url) = char.hooks.get(&ctx.channel_id().get()) {
        let user = FakeUserMaker::new(&ctx).existing(&hook_url, &char.display_name, &char.avatar_url).await;
        if let Ok(user) = user {
            user.send(WebhookMessage::Text(content.to_owned())).await?;
        } else if let Err(err) = user {
            match err {
                BotError::FakeUser(err) => {
                    match err {
                        FakeUserError::InvalidWebhook { .. } => {
                            has_existing_hook = false;

                            // Removing any invalid webhook
                            if let Ok(mut write) = ctx.data().servers.write() {
                                let server_write = write.get_mut(&guild_id.get()).unwrap();
                                let char_write = server_write.characters.get_mut(id).unwrap();
                                char_write.hooks.remove(&ctx.channel_id().get());
                            }

                            // debug
                            let name = ctx.channel_id().name(ctx.http()).await;
                            if let Ok(name) = name {
                                println!("Invalid webhook found. Removing {}..", name);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    } else {
        has_existing_hook = false;
    }

    // Creating a new hook instead
    if !has_existing_hook {
        let user = FakeUserMaker::new(&ctx)
            .new_hook(&char.display_name, &char.avatar_url, false).await?;
        let hook_url = user.get_webhook_url().unwrap();
        user.send(WebhookMessage::Text(content.to_owned())).await?;

        // Adding the new hook to our data
        if let Ok(mut write) = ctx.data().servers.write() {
            let server_write = write.get_mut(&guild_id.get()).unwrap();
            let char_write = server_write.characters.get_mut(id).unwrap();
            char_write.hooks.insert(ctx.channel_id().get(), hook_url.clone());
        } else {
            return Err(BotError::Str("Unable to unlock server write lock"));
        }
    }

    // Sending out the message
    println!(
        "{User} said \"{content}\"",
        User = ctx.author().display_name()
    );

    consume_interaction(ctx).await;
    Ok(())
}

/*async fn autocomplete<'a>(ctx: Context<'a>, partial: &str) -> Result<Vec<String>, Error> {
    Ok(Vec::new())
}*/