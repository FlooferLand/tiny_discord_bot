#![allow(unused)]

use std::sync::Arc;
use poise::serenity_prelude::{Guild, GuildId, ImageHash};
use crate::Context;
use crate::serenity::builder::ExecuteWebhook;
use crate::serenity::http::Http;
use crate::serenity::json::{json, Value};
use crate::serenity::model::channel::Message;
use crate::serenity::model::id::{ChannelId, UserId};
use crate::serenity::model::prelude::User;

// let name = format!(
//     "{name}{0}˞",
//     String::from(" ").repeat(79 - name.len())
// );

type WebhookMessageBuilder = ExecuteWebhook;
pub enum WebhookMessage {
    Text(String),
    Builder(WebhookMessageBuilder)
}

pub enum Kind {
	NewHook {
		name: String,
		avatar_url: String,
	},
	ExistingHook {
		webhook_url: String
	}
}

pub struct FakeUser {
	data: Kind
}

// Methods
impl FakeUser {
	async fn get_name(http: &Http, guild: GuildId, user: &User) -> String {
		match user.nick_in(http, guild).await {
			Some(nick) => nick,
			None => user.display_name().to_string()
		}
	}

	fn get_avatar_url(user: User) -> String {
		match user.avatar_url() {   
			Some(url) => url,
			None => user.default_avatar_url(),
		}
	}

    // Sending messages (methods)
    pub async fn send(&self, http: &Http, channel: ChannelId, content: WebhookMessage) {
		let (webhook, content) = match &self.data {
			Kind::NewHook { name, avatar_url } => {
				let mut init = json!({ "name": name });
				let mut map = init.as_object_mut().unwrap();
				let webhook = http.create_webhook(
					channel,
					&Value::Object(map.clone()),
					None
				).await.unwrap();
				let content = match content {
					WebhookMessage::Text(text) =>
						ExecuteWebhook::new()
							.content(text)
							.avatar_url(avatar_url),
					WebhookMessage::Builder(b) => b
						.avatar_url(avatar_url),
				};
				(webhook, content)
			}
			
			Kind::ExistingHook { webhook_url } => {
				let webhook = http.get_webhook_from_url(&*webhook_url).await.unwrap();
				let content = match content {
					WebhookMessage::Text(text) => ExecuteWebhook::new().content(text),
					WebhookMessage::Builder(b) => b,
				};
				(webhook, content)
			}
		};

        // Running the webhook
        webhook
            .execute(&http, true, content)
            .await
            .expect("Executing webhook");
    }
}

// Constructors
impl FakeUser {
    pub async fn new(http: &Http, name: &str, avatar_url: &str) -> Self {
	    Self { 
		    data: Kind::NewHook {
			    name: name.to_string(),
			    avatar_url: avatar_url.to_string()
		    }
	    }
    }

	pub async fn from_user(http: &Http, guild: GuildId, user: User) -> Self {
		Self {
			data: Kind::NewHook {
				name: Self::get_name(&http, guild, &user).await,
				avatar_url: Self::get_avatar_url(user)
			}
		}
	}

	pub async fn from_url(http: &Http, url: &str) -> Self {
		Self {
			data: Kind::ExistingHook {
				webhook_url: url.to_string()
			}
		}
	}

    pub async fn from_userid(http: &Http, guild: GuildId, user_id: UserId) -> Self {
        let user = user_id.to_user(&http).await.unwrap();
        Self::from_user(http, guild, user).await
    }

    /// Sends a message while replacing its text, pretending to be the user that sent the message
    pub async fn send_replace_text(http: &Http, message: Message, new_text: String) {
        message.delete(&http).await.expect("Could not delete message!");
        Self::from_user(http, message.guild_id.unwrap_or_default(), message.author.clone()).await
            .send(http, message.channel_id, WebhookMessage::Text(message.content.to_owned()))
            .await;
    }
}
