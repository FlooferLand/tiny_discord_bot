mod commands;
mod fake_user;
mod event_handler;
mod data;
mod error;
mod util;
mod log;

use crate::commands::char::char_use::say_as;
use crate::commands::info::bot_info;
use crate::commands::save::save;
use crate::data::load_data;
use crate::data::servers::Server;
use crate::error::{BotError, BotErrorExt};
use crate::event_handler::{error_handler, event_handler};
use poise::serenity_prelude::{ActivityData, OnlineStatus};
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::commands::char::char;

struct BotData {
    pub servers: Arc<RwLock<HashMap<u64, Server>>>,
}

type Context<'a> = poise::Context<'a, BotData, BotError>;

#[tokio::main]
async fn main() {
    let token = std::env::var("TINY_BOT_TOKEN").expect("env 'TINY_BOT_TOKEN' should be set");
    let intents = serenity::GatewayIntents::privileged();
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                bot_info(),
                char(), say_as(),
                save()
            ],
            on_error: |error| Box::pin(error_handler(error)),
            .. Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands
                poise::builtins::register_globally(ctx, &framework.options().commands).await.bot_err()?;

                // Loading in data
                #[allow(unused_parens)]
                let (servers) = load_data();

                // Status
                println!("Bot online! ({})", Utc::now().format("%Y/%m/%d"));
                println!("Loaded {} servers!", servers.len());
                ctx.set_presence(Some(ActivityData::custom("2025 Policy :3c")), OnlineStatus::Online);

                // Data
                Ok(BotData {
                    servers: Arc::new(RwLock::new(servers)),
                })
            })
        })
        .build();

    // Starting the server
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();
    client.start().await.unwrap();

    // Graceful shutdown
    let shard_manager = client.shard_manager.clone();
    let _ = ctrlc::set_handler(move || {
        let shard_runners = shard_manager.runners.blocking_lock();
        for (_id, runner) in shard_runners.iter() {
            runner.runner_tx.set_status(OnlineStatus::Offline);
        }
        println!("Bot stopped!");
        std::process::exit(0);
    });
}
