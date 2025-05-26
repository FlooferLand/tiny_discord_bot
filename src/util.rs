use crate::Context;
use poise::CreateReply;

/// Discord has an annoying need for apps to send a reply message to users.
/// Calling this function at the end of a command removes that
pub async fn swallow_interaction<'a>(ctx: Context<'a>) {
	let _ = ctx.defer_ephemeral().await;
	let message = ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await;
	if let Ok(message) = message {
		message.delete(ctx).await.unwrap()
	}
}
