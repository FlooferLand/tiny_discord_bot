use poise::{BoxFuture, FrameworkError};
use crate::serenity;
use poise::serenity_prelude::{CacheHttp, FullEvent, MessageBuilder, RoleId};
use crate::{BotData, BotError};
use crate::data::servers::Server;

pub async fn error_handler(error: FrameworkError<'_, BotData, BotError>) {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            let text = match error {
                BotError::String(value) => value,
                BotError::Str(value) => value.to_string(),
                e => MessageBuilder::new().push_mono(e.to_string()).build()
            };
            let message = MessageBuilder::new()
                .push_bold("ERROR:").push(" ").push_safe(text)
                .build();
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(message)
            ).await.map(|_| ()).unwrap()
        },
        error => poise::builtins::on_error(error).await.unwrap(),
    };
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, BotData, BotError>,
    data: &BotData,
) -> Result<(), BotError> {
    match event {
        FullEvent::GuildCreate { guild, .. } => {
            if let Ok(mut server_write) = data.servers.write() {
                server_write.insert(guild.id.get(), Server::default());
            }
        }
        FullEvent::Message { new_message } => {
            /*let Some(server) = new_message.guild_id.and_then(|a| data.servers.get(&a.get())) else {
                return Err(anyhow!("Failed to find guild settings for guild id '{}'", new_message.guild_id.unwrap_or_default()));
            };
            
            // Moderator role lel
            let mod_role = RoleId::new(server.roles.moderator);
            if new_message.mention_roles.contains(&mod_role) {
                data.bob.reply_direct(ctx.http(), new_message).await;
            }*/
        }
        _ => {}
    }
    Ok(())
}
