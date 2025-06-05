use crate::Context;
use std::cmp::Ordering;

// TODO: Try to move to use [`AutocompleteChoice`] and a name system,
//       so the user won't have to remember the bot ID separately from the display name
/// Autocompletes character IDs
pub async fn character<'a>(ctx: Context<'a>, partial: &str) -> Vec<String> {
	let Some(guild_id) = ctx.guild_id() else {
		return Vec::new();
	};
	let servers = &ctx.data().servers;
	let Some(server) = servers.get(&guild_id.get()) else {
		return Vec::new();
	};
	let mut chars = server.characters
		.iter()
		.filter_map(|e| {
			let e = e.key();
			if e.contains(&partial.to_ascii_lowercase()) {
				Some(e.clone())
			} else {
				None
			}
		})
		.collect::<Vec<String>>();
	if chars.len() > 10 {
		chars.sort_by(|a, b| {
			let a = strsim::jaro_winkler(a, partial);
			let b = strsim::jaro_winkler(b, partial);
			a.partial_cmp(&b).unwrap_or(Ordering::Equal)
		});
	}

	chars
}