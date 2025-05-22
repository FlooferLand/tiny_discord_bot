mod commands;
mod fake_user;
mod event_handler;
mod asset_manager;
mod data;
mod error;

use crate::commands::char_use::say_as;
use crate::commands::info::info;
use crate::data::servers::Server;
use crate::event_handler::event_handler;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ActivityData, OnlineStatus, ShardId, ShardManager, ShardRunnerInfo};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use poise::serenity_prelude::prelude::{SerenityError, TypeMapKey};
use crate::commands::save::save;
use crate::error::{BotError, BotErrorExt};

struct BotData {
    pub servers: Arc<RwLock<HashMap<u64, Server>>>,
}

type Context<'a> = poise::Context<'a, BotData, BotError>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("env 'DISCORD_TOKEN' should be set");
    let intents = serenity::GatewayIntents::privileged();
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                info(), say_as(), save()
            ],
            // on_error: error_handler,
            .. Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands
                poise::builtins::register_globally(ctx, &framework.options().commands).await.bot_err()?;

                // Loading server data
                let mut servers = HashMap::new();
                let _ = std::fs::create_dir_all("./assets/data/servers/");
                let servers_dir = std::fs::read_dir("./assets/data/servers/").unwrap();
                for entry in servers_dir {
                    let Ok(dir) = entry else { continue };
                    let Ok(server_text) = std::fs::read_to_string(dir.path()) else {
                        panic!("Failed to parse server data");
                    };
                    let Ok(server_id) = dir.path().file_stem().unwrap_or(dir.file_name().as_os_str()).to_string_lossy().parse::<u64>() else {
                        panic!("Failed to parse server ID");
                    };;
                    match serde_yml::from_str::<Server>(server_text.as_str()) {
                        Ok(server_data) => {
                            servers.insert(server_id, server_data);
                        }
                        Err(err) => {
                            panic!("Failed to deserialize server data: {err}");
                        }
                    };
                }

                // Status
                println!("Bot online!");
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
}