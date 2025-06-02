use std::fmt::{Debug, Display};
use strsim::damerau_levenshtein;

#[derive(Debug, Clone)]
pub struct FuzzyPattern<'a, Enum> {
	pub id: &'a Enum,
	pub entries: Vec<String>
}

#[derive(Debug)]
pub struct Fuzzy<'a, Enum> {
	patterns: Vec<FuzzyPattern<'a, Enum>>
}
impl<'a, Enum> Fuzzy<'a, Enum> where Enum: Debug {
	pub fn new() -> Self {
		Self {
			patterns: Vec::new()
		}
	}

	pub fn add<S>(&mut self, id: &'a Enum, entries: Vec<S>) where S: Display {
		let entries = entries.iter().map(|a| a.to_string()).collect::<Vec<String>>();
		let entry = FuzzyPattern { id, entries };
		self.patterns.push(entry);
	}

	pub fn add_varied<S>(&mut self, id: &'a Enum, entries: Vec<S>) where S: Display {
		let mut out = Vec::with_capacity(entries.len() * 2);
		for entry in &entries {
			out.push(entry.to_string());
			out.push(entry.to_string().replace("\'", ""));
		}
		self.add(id, out);
	}

	/// Returns `Some((pattern, score))` if the best matching score is over `threshold`,
	/// otherwise returns `None`.
	pub fn best_match(
		&self,
		text: &str,
		threshold: f32,
	) -> Option<(&FuzzyPattern<'a, Enum>, f32)> {
		let mut best: Option<(&FuzzyPattern<'a, Enum>, f32)> = None;

		for pattern in &self.patterns {
			let best_score = pattern
				.entries
				.iter()
				.map(|entry| {
					let dist = if text.contains(entry) {
						0.2
					} else {
						damerau_levenshtein(text, entry) as f32
					};
					let max_len = (text.len().max(entry.len())) as f32;
					let norm = (dist / max_len).clamp(0.0, 1.0);
					1.0 - norm
				})
				.fold(0.0, f32::max);

			println!("{:#?}{}", pattern.id, best_score);
			
			match best {
				None => best = Some((pattern, best_score)),
				Some((_, prev_score)) if best_score > prev_score => {
					best = Some((pattern, best_score));
				}
				_ => {}
			}
		}

		best.and_then(|(pat, score)| {
			if score >= threshold {
				Some((pat, score))
			} else {
				None
			}
		})
		}
}
