use crate::fake_user::{FakeUser, WebhookMessage};
use crate::{Context, BotError};
use poise::CreateReply;
use poise::serenity_prelude::json::{json, Value};
use crate::error::{BotErrorMsgExt, BotErrorExt};

#[poise::command(slash_command, rename="sayas", aliases("say", "sayas", "say_as"))]
pub async fn say_as(
    ctx: Context<'_>,
    #[description = "ID"] id: String,
    #[description = "Content"] content: String
) -> Result<(), BotError> {
    let channel_id = ctx.channel_id().get();
    let guild_id = ctx.guild_id().bot_err("No guild ID found")?;
    let hook_block = {
        let (display_name, avatar_url, has_hook_for_channel) = {
            let server_read = ctx.data().servers.read().bot_err()?;
            let server = server_read.get(&guild_id.get()).bot_err("Unable to find server")?;
            let char_read = server.characters.get(&id).bot_err("Failed to find character")?;
            Ok::<(String, String, bool), String>((char_read.display_name.clone(), char_read.avatar_url.clone(), char_read.hooks.contains_key(&channel_id)))
        }.unwrap();

        if has_hook_for_channel {
            let server_read = ctx.data().servers.read().bot_err()?;
            let server = server_read.get(&guild_id.get()).unwrap();
            let char_read = server.characters.get(&id).unwrap();
            Ok(char_read.hooks.get(&channel_id).unwrap().clone())
        } else {
            let mut init = json!({ "name": display_name, "avatar_url": avatar_url });
            let map = init.as_object_mut().unwrap();
            let webhook = ctx.http().create_webhook(
                ctx.channel_id(),
                &Value::Object(map.clone()),
                None
            ).await.bot_err()?;
            let Ok(hook_url) = webhook.url() else {
                return Err(BotError::Str("Failed getting webhook URL after creation"))
            };
            if let Ok(mut write) = ctx.data().servers.try_write() {
                let server_write = write.get_mut(&guild_id.get()).unwrap();
                let char_write = server_write.characters.get_mut(&id).unwrap();
                char_write.hooks.insert(channel_id, hook_url.clone());
                Ok(hook_url)
            } else {
                Err(BotError::Str("Unable to unlock server write lock"))
            }
        }
    };

    if let Ok(hook) = hook_block {
        let user = FakeUser::from_url(ctx.http(), &hook).await;

        println!(
            "{User} said \"{content}\"",
            User = ctx.author().display_name()
        );
        user.send(
            ctx.http(),
            ctx.channel_id(),
            WebhookMessage::Text(content)
        ).await;

    };
    ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await.bot_err()?;
    Ok(())
}

/*async fn autocomplete<'a>(ctx: Context<'a>, partial: &str) -> Result<Vec<String>, Error> {
    Ok(Vec::new())
}*/