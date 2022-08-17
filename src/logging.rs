use std::error::Error;
use std::fs;
use std::path::Path;

use chrono::Local;
use log::{error, info, LevelFilter, warn};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

#[derive(Copy, Clone)]
pub enum LogLevel {
	Info,
	Warning,
	Error,
}

/// Universal logging fn, should be used everywhere during unattended runtime
pub fn print_log(input: &str, log_level: LogLevel) {
	println!("{} {}", chrono::Local::now().naive_local(), input);
	match log_level {
		LogLevel::Info => {
			info!("{}", input);
		}
		LogLevel::Warning => {
			warn!("{}", input);
		}
		LogLevel::Error => {
			error!("{}", input);
		}
	}
}

/// Sets up new empty log file, archives old log file(s)
pub fn init_log() -> Result<(), Box<dyn Error>> {
	if Path::new("log/latest.log").exists() {
		let now = Local::now().format("%Y_%m_%d_%H-%M-%S").to_string();
		fs::rename("log/latest.log", format!("log/old/{}.log", now))?;
	}

	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{l} {d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
		.build("log/latest.log")?;

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.build(Root::builder()
			.appender("logfile")
			.build(LevelFilter::Info))?;

	log4rs::init_config(config).unwrap();
	Ok(())
}
