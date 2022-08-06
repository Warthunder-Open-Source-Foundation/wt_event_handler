pub fn print_log(input: &str, log_level: u8) {
	println!("{} {}", chrono::Local::now().naive_local(), input);
	match log_level {
		2 => {
			info!("{}", input);
		}
		1 => {
			warn!("{}", input);
		}
		_ => {
			error!("{}", input);
		}
	}
}
