mod commands;
mod fake_user;
mod event_handler;
mod data;
mod error;
mod util;
mod logger;
mod fuzzy;
mod autocomplete;

use std::collections::HashSet;
use crate::commands::char::char_use::say_as;
use crate::commands::info::bot_info;
use crate::commands::save::save;
use crate::data::load_data;
use crate::data::servers::Server;
use crate::error::{BotError, BotErrorExt};
use crate::event_handler::{error_handler, event_handler};
use poise::serenity_prelude::{ActivityData, OnlineStatus, UserId};
use poise::serenity_prelude as serenity;
use std::sync::Arc;
use chrono::Utc;
use dashmap::DashMap;
use dotenvy::dotenv;
use log::{error, info};
use tokio::sync::RwLock;
use crate::commands::char::char;
use crate::commands::{post_command, pre_command};
use crate::commands::help::help;
use crate::logger::Logger;

pub type ArcLock<T> = Arc<RwLock<T>>;

struct BotData {
    pub servers: DashMap<u64, Server>,
}

type Context<'a> = poise::Context<'a, BotData, BotError>;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let token = std::env::var("TINY_BOT_TOKEN").expect("env 'TINY_BOT_TOKEN' should be set");
    let intents = serenity::GatewayIntents::all();
    Logger::init().unwrap();

    // Creating the poise framework instance
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                bot_info(),
                char(), say_as(),
                save(), help()
            ],
            pre_command: |ctx| Box::pin(pre_command(ctx)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            on_error: |error| Box::pin(error_handler(error)),
            owners: {
                let mut set = HashSet::new();
                set.insert(UserId::new(792764829689315349));
                set
            },
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
                println!();
                info!("Bot online! ({})", Utc::now().format("%Y/%m/%d"));
                info!("Loaded {} servers!", servers.len());
                ctx.set_presence(Some(ActivityData::custom("2025 Policy :3c")), OnlineStatus::Online);

                // Data
                Ok(BotData {
                    servers,
                })
            })
        })
        .build();

    // Creating the client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();

    // Graceful shutdown
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        {
            let shard_runners = shard_manager.runners.lock().await;
            for (_id, runner) in shard_runners.iter() {
                runner.runner_tx.set_status(OnlineStatus::Offline);
            }
        }
        shard_manager.shutdown_all().await;
        info!("Bot stopped!");
    });

    // Starting the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
