#![allow(unused)]
use chrono::Utc;

#[macro_export]
macro_rules! debug {
	($($arg:tt)*) => {
		log("DEBUG", $crate::fmt::format($crate::__export::format_args!($($arg)*)))
	}
}

#[macro_export]
macro_rules! info {
	($($arg:tt)*) => {
		log("INFO", $crate::core::fmt::format($crate::__export::format_args!($($arg)*)))
	}
}

#[macro_export]
macro_rules! warning {
	($($arg:tt)*) => {
		log("WARN", $crate::fmt::format($crate::__export::format_args!($($arg)*)))
	}
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {
		log("ERROR", $crate::fmt::format($crate::__export::format_args!($($arg)*)))
	}
}

fn log(tag: &str, text: &str) {
	let date = Utc::now().format("%Y-%m-%d");
	println!("[{date}] [{tag}]\t{text}");
}
