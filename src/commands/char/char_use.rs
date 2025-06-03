use crate::fake_user::{FakeUserError, FakeUserMaker, WebhookMessage};
use crate::util::consume_interaction;
use crate::{read_server, write_server, BotError, Context};
use std::cmp::Ordering;

/// Use a character to send a message (shorthand for `char_use`)
#[poise::command(slash_command, rename="sayas")]
pub async fn say_as(
    ctx: Context<'_>,
    #[description = "ID"] #[autocomplete="id_complete"] id: String,
    #[description = "Content"] content: String
) -> Result<(), BotError> {
    Box::pin(inner(ctx, id, content)).await
}

/// Use a character to send a message
#[poise::command(slash_command, rename="use")]
pub(super) async fn char_use(
    ctx: Context<'_>,
    #[description = "ID"] #[autocomplete="id_complete"] id: String,
    #[description = "Content"] content: String
) -> Result<(), BotError> {
    Box::pin(inner(ctx, id, content)).await
}

#[allow(dead_code)]
async fn inner(ctx: Context<'_>, id: String, content: String) -> Result<(), BotError> {
    let id = id.trim();

    // Getting the character
    let char = read_server!(ctx, characters => {
        characters.get(id).bot_err("Unable to find character")?.clone()
    });

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
                            write_server!(ctx, characters => {
                                let mut char = characters.get_mut(id).unwrap();
                                char.hooks.remove(&ctx.channel_id().get());
                            });

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
        write_server!(ctx, characters => {
            let mut char = characters.get_mut(id).bot_err("Unable to find character")?;
            char.hooks.insert(ctx.channel_id().get(), hook_url.clone());
        });
    }

    // Sending out the message
    println!(
        "{User} said \"{content}\"",
        User = ctx.author().display_name()
    );

    consume_interaction(ctx).await;
    Ok(())
}

async fn id_complete<'a>(ctx: Context<'a>, partial: &str) -> Vec<String> {
    let Some(guild_id) = ctx.guild_id() else {
        return Vec::new();
    };
    let servers = &ctx.data().servers;
    let Some(server) = servers.get(&guild_id.get()) else {
        return Vec::new();
    };
    let mut chars = (server.characters.clone())
        .iter()
        .map(|entry| entry.key().to_owned())
        .collect::<Vec<String>>();
    chars.sort_by(|a, b| {
        let a = strsim::jaro_winkler(a, partial);
        let b = strsim::jaro_winkler(b, partial);
        a.partial_cmp(&b).unwrap_or(Ordering::Equal)
    });
    chars
}