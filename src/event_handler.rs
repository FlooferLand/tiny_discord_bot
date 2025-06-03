use log::debug;
use crate::data::servers::Server;
use crate::serenity;
use crate::{BotData, BotError};
use poise::serenity_prelude::{FullEvent, MessageBuilder};
use poise::FrameworkError;
use crate::error::BotErrorExt;
use crate::fuzzy::Fuzzy;

pub async fn error_handler(error: FrameworkError<'_, BotData, BotError>) {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            let text = match error {
                BotError::String(value) => value,
                BotError::Str(value) => value.to_string(),
                e => MessageBuilder::new().push_mono(e.to_string()).build()
            };
            let message = MessageBuilder::new()
                .push_bold("ERROR:").push(" ").push_safe(&text)
                .build();

            debug!("Skill issue [{}]: \"{text}\"", ctx.author().name);

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
            debug!("Found guild \"{}\"", guild.name);
        }
        FullEvent::Message { new_message } => {
            if new_message.author.id == ctx.http.get_current_user().await.bot_err()?.id {
                return Ok(());
            }
            
            //let silly_messages = read_server(ctx, |server| server.config.silly_messages.ok());

            if new_message.channel_id == 1057519403408822283 {
                println!("Message: '{}'", new_message.content);
                let user_text = new_message.content.replace(|c: char| !c.is_ascii(), "");

                #[derive(Debug)]
                enum FuzzyEnum {
                    Banning,
                    ModMention
                }
                let mut fuzzy = Fuzzy::new();
                fuzzy.add_varied(&FuzzyEnum::Banning, vec![
                    "i'm banning you",
                    "you're getting banned",
                ]);
                fuzzy.add_varied(&FuzzyEnum::ModMention, vec![
                    "bob",
                    "moderators",
                    "mods",
                ]);
                if let Some((winning_pattern, score)) = fuzzy.best_match(&user_text, 0.3) {
                    new_message.reply(&ctx.http, format!("```json\nSCORE:{score},\n{winning_pattern:#?}\n```")).await.bot_err()?;
                }
            }

            // Moderator role lel
            /*let mod_role = RoleId::new(server.roles.moderator);
            if new_message.mention_roles.contains(&mod_role) {
                data.bob.reply_direct(ctx.http(), new_message).await;
            }*/
        }
        _ => {}
    }
    Ok(())
}
