use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::sync::RwLock;

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
lazy_static! {
	static ref todays_date: RwLock<DateTime<Utc>> = RwLock::new(Utc::now());
}

pub struct Logger;
impl Logger {
	pub fn init() -> Result<(), SetLoggerError> {
		log::set_logger(&Logger)?;
		log::set_max_level(LOG_LEVEL);
		Ok(())
	}

	fn can_log(module: &str, level: Level) -> bool {
		if module.starts_with(env!("CARGO_PKG_NAME")) {
			return level <= LOG_LEVEL
		}
		level == LevelFilter::Error
			|| level == LevelFilter::Warn
	}
}

impl Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		Self::can_log(metadata.target(), metadata.level())
	}

	fn log(&self, record: &Record) {
		let module = record.module_path().unwrap_or(record.target());
		if !Self::can_log(module, record.level()) {
			return;
		}

		// Formatting
		let msg = format!(
			"[{Date}] [{Level}]\t{Args}",
			Date = Utc::now().format("%H:%M:%S"),
			Level = record.level(),
			Args = record.args()
		);

		// Time separator
		let latest = Utc::now();
		let same_date = match todays_date.read() {
			Ok(today) => (latest - *today).num_days() == 0,
			Err(_) => false
		};
		if !same_date {
			println!("\n\n### {} ###\n", latest.format("%Y/%m/%d"));
			if let Ok(mut today) = todays_date.write() {
				*today = latest;
			}
		}

		// Printing
		if record.level() == LevelFilter::Error || record.level() == LevelFilter::Warn {
			eprintln!("{msg}");
		} else {
			println!("{msg}");
		}
	}

	fn flush(&self) {}
}
