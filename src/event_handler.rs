use poise::FrameworkError;
use crate::serenity;
use poise::serenity_prelude::{CacheHttp, FullEvent, RoleId};
use crate::{BotData, BotError};
use crate::data::servers::Server;

pub async fn error_handler(error: FrameworkError<'_, BotData, BotError>) -> Box<()> {
    Box::new(())
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
