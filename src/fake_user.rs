use crate::error::{BotError, BotErrorExt, BotErrorMsgExt};
use crate::serenity::builder::ExecuteWebhook;
use crate::serenity::http::Http;
use crate::serenity::model::channel::Message;
use crate::serenity::model::id::ChannelId;
use crate::serenity::model::prelude::User;
use crate::Context;
use poise::serenity_prelude::{CreateAttachment, CreateWebhook, GuildId};

// let name = format!(
//     "{name}{0}˞",
//     String::from(" ").repeat(79 - name.len())
// );

type WebhookMessageBuilder = ExecuteWebhook;

#[allow(dead_code)]
pub enum WebhookMessage {
    Text(String),
    Builder(WebhookMessageBuilder)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum FakeUserError {
	InvalidWebhook { webhook_url: String }
}

#[allow(unused)]
pub struct FakeUser<'a> {
	webhook_url: Option<String>,
	avatar_url: String,
	name: String,
	temporary: bool,

	// From the FakeUserMaker
	http: &'a Http,
	channel: ChannelId
}

// Methods
#[allow(dead_code)]
impl<'a> FakeUser<'a> {
	async fn get_user_name(http: &Http, guild: GuildId, user: &User) -> String {
		match user.nick_in(http, guild).await {
			Some(nick) => nick,
			None => user.display_name().to_string()
		}
	}
	fn get_user_avatar_url(user: User) -> String {
		match user.avatar_url() {   
			Some(url) => url,
			None => user.default_avatar_url(),
		}
	}

	/// Returns `None` if using a temporary hook, as they get automatically freed after usage
	pub fn get_webhook_url(&self) -> Option<String> {
		match self.temporary {
			true => None,
			false => Some(self.webhook_url.to_owned()?),
		}
	}

    /// Sending the message.
    pub async fn send(&self, content: WebhookMessage) -> Result<(), BotError> {
	    let hook_url = self.get_webhook_url().bot_err("Failed to get webhook URL")?;
		let webhook = self.http.get_webhook_from_url(&hook_url).await.bot_err()?;

	    let content = match content {
		    WebhookMessage::Text(text) => ExecuteWebhook::new().content(text),
		    WebhookMessage::Builder(b) => b,
	    };

        // Running the webhook
        webhook
            .execute(self.http, true, content)
            .await.bot_err()?;
	    if self.temporary {
		    webhook.delete(self.http).await.bot_err()?;
	    }
	    Ok(())
    }

	/// Sends a message while replacing its text, pretending to be the user that sent the message
	pub async fn user_replace_text(ctx: &'a Context<'_>, message: Message, new_text: String) -> Result<(), BotError> {
		message.delete(&ctx.http()).await.bot_err()?;
		FakeUserMaker::new(ctx)
			.user(message.author.clone()).await?
			.send(WebhookMessage::Text(new_text))
			.await
	}
}

// Constructors
pub struct FakeUserMaker<'a> {
	http: &'a Http,
	channel: ChannelId
}
#[allow(dead_code)]
impl<'a> FakeUserMaker<'a> {
	pub fn new(ctx: &'a Context) -> Self {
		Self {
			http: ctx.http(),
			channel: ctx.channel_id().clone()
		}
	}

    pub async fn new_hook(self, name: &str, avatar_url: &str, temporary: bool) -> Result<FakeUser<'a>, BotError> {
	    let avatar = CreateAttachment::url(self.http, &avatar_url).await.bot_err()?;
	    let builder = CreateWebhook::new(name).avatar(&avatar);
	    let webhook = self.channel.create_webhook(self.http, builder).await.bot_err()?;
	    Ok(FakeUser {
		    temporary,
		    name: name.to_string(),
		    avatar_url: avatar_url.to_string(),
		    webhook_url: Some(webhook.url().bot_err()?),
		    http: self.http,
		    channel: self.channel
	    })
    }

	/// **NOTE:** If the webhook provided doesn't exist,
	/// it will return [`FakeUserError::InvalidWebhook`].
	pub async fn existing(self, webhook_url: &str, backup_name: &str, backup_avatar_url: &str) -> Result<FakeUser<'a>, BotError> {
		if let Err(_) = self.http.get_webhook_from_url(webhook_url).await {
			return Err(BotError::FakeUser(FakeUserError::InvalidWebhook { webhook_url: webhook_url.to_string() }))
		};
		Ok(FakeUser {
			temporary: false,
			name: backup_name.to_string(),
			webhook_url: Some(webhook_url.to_string()),
			avatar_url: backup_avatar_url.to_string(),
			http: self.http,
			channel: self.channel
		})
	}

	pub async fn user(self, user: User) -> Result<FakeUser<'a>, BotError> {
		let guild = self.channel.to_channel(self.http).await.bot_err()?.guild().bot_err("Failed to find guild")?.guild_id;
		Ok(FakeUser {
			webhook_url: None,
			temporary: true,
			name: FakeUser::get_user_name(&self.http, guild, &user).await,
			avatar_url: FakeUser::get_user_avatar_url(user),
			http: self.http,
			channel: self.channel
		})
	}

    /*pub async fn userid(self, user_id: UserId) -> Result<FakeUser<'a>, BotError> {
        let user = user_id.to_user(&self.http).await.unwrap();
        self.user(user).await
    }*/
}
