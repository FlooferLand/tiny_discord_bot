use poise::builtins::HelpConfiguration;
use crate::Context;
use crate::error::{BotError, BotErrorExt};

/// Lists all commands and some helpful info
#[poise::command(slash_command, category = "Utility")]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Command to get help for"] #[rest] mut command: Option<String>,
) -> Result<(), BotError> {
	if ctx.invoked_command_name() != "help" {
		command = match command {
			Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
			None => Some(ctx.invoked_command_name().to_string()),
		};
	}

	let config = HelpConfiguration {
		include_description: true,
		show_subcommands: true,
		show_context_menu_commands: false,
		ephemeral: true,

		..Default::default()
	};
	poise::builtins::help(ctx, command.as_deref(), config).await.bot_err()
}