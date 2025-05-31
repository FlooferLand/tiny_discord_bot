#![allow(unused)]
use std::fmt::{Display, Formatter};
use std::sync::LockResult;
use poise::serenity_prelude::prelude::SerenityError;
use crate::fake_user::FakeUserError;

#[derive(Debug)]
pub enum BotError {
	Serenity(SerenityError),
	String(String),
	Str(&'static str),
	FakeUser(FakeUserError)
}

impl Display for BotError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[macro_export]
macro_rules! err_fmt {
     ($($arg:tt)*) => {
        BotError::String(format!($($arg)*))
    }
 }


// Error conversion
pub trait BotErrorExt<V> {
	fn bot_err(self) -> Result<V, BotError>;
}
pub trait BotErrorMsgExt<V> {
	fn bot_err(self, err: &str) -> Result<V, BotError>;
}
impl<V> BotErrorExt<V> for Result<V, SerenityError> {
	fn bot_err(self) -> Result<V, BotError> {
		self.map_err(|err| BotError::Serenity(err))
	}
}
impl<V> BotErrorExt<V> for Result<V, String> {
	fn bot_err(self) -> Result<V, BotError> {
		self.map_err(|err| BotError::String(err))
	}
}
impl<V> BotErrorMsgExt<V> for Option<V> {
	fn bot_err(self, err: &str) -> Result<V, BotError> {
		match self {
			Some(v) => Ok(v),
			None => Err(BotError::String(err.to_string()))
		}
	}
}
impl<V> BotErrorExt<V> for LockResult<V> {
	fn bot_err(self) -> Result<V, BotError> {
		match self {
			Ok(v) => Ok(v),
			Err(err) => Err(BotError::String(err.to_string()))
		}
	}
}

// Result utilities
pub trait OkExt<O, E> {
	fn ok(self) -> Result<O, E>;
}
impl<O, E> OkExt<O, E> for O {
	fn ok(self) -> Result<O, E> {
		Ok(self)
	}
}

// Option utilities
pub trait SomeExt<O> {
	fn some(self) -> Option<O>;
}

impl<O> SomeExt<O> for O {
	fn some(self) -> Option<O> {
		Some(self)
	}
}
