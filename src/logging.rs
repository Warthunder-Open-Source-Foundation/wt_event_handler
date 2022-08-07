use log::{error, info, warn};

#[derive(Copy, Clone)]
pub enum LogLevel {
	Info,
	Warning,
	Error,
}

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
