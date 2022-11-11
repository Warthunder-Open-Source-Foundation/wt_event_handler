#![feature(if_let_guard)]
#![feature(once_cell)]
#![feature(async_closure)]
#![feature(type_ascription)]

// Reason: Keeps type name collisions at bay
#![allow(clippy::module_name_repetitions)]

// Reason: Just makes unwrap_or calls much more verbose than they need to be
#![allow(clippy::or_fun_call)]

use std::{fs, io};
use std::io::stdout;
use std::process::exit;
use std::time::Instant;

use lazy_static::{initialize, lazy_static};
use rand::Rng;
use tracing::{Level, warn};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::error::NewsError;
use crate::fetch_loop::fetch_loop;
use crate::json::webhooks::CrashHook;
use crate::json::webhooks::WebhookAuth;
use crate::menu_options::{add_webhook, remove_webhook, test_hook};

mod webhook_handler;
mod scrapers;
mod json;
mod menu_options;
mod fetch_loop;
mod embed;
mod error;
mod timeout;
mod statistics;
mod api;

const TOKEN_PATH: &str = "assets/discord_token.json";


lazy_static! {
	pub static ref WEBHOOK_AUTH: WebhookAuth = {
		let raw = fs::read(TOKEN_PATH).unwrap();
		let json: WebhookAuth = serde_json::from_slice(&raw).unwrap();
		json
	};
	pub static ref PANIC_INFO: CrashHook = {
		WEBHOOK_AUTH.crash_hook[0].clone()
	};
	pub static ref SHUTDOWN_KEY: String =  {
		rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
	};
	pub static ref BOOT_TIME: Instant =  {
		Instant::now()
	};
}

#[tokio::main]
async fn main() -> Result<(), NewsError> {
	// Loads statics
	initialize(&WEBHOOK_AUTH);
	initialize(&PANIC_INFO);
	initialize(&SHUTDOWN_KEY);

	println!("Emergency shutdown param: localhost:8082/settings/shutdown/{}", *SHUTDOWN_KEY);

	let mut line = String::new();
	let mut hooks = true;

	println!("Please select a start profile:\n\
	1. Regular initialization\n\
	2. Boot without sending hooks\n\
	3. Add new webhook-client\n\
	4. Remove a webhook\n\
	5. Test webhook client / channel");
	io::stdin().read_line(&mut line).expect("failed to read from stdin");

	// LOGGING CONVENTION
	// Trace - unused
	// Debug - Typically Unused
	// Info - Used for things that happen in guaranteed intervals
	// Warn - Used for irregular occurrences such as finding news
	// Error - Any (un)recoverable error blocking part of regular execution or halting it entirely

	// Both trace and Debug are not logged to files or stdout

	let debug_file = rolling::daily("./log/debug", "debug").with_filter(|x| *x.level() == Level::INFO);
	let warn_file = rolling::never("./log/warning", "warnings").with_filter(|x| *x.level() <= Level::WARN);
	let all_files = debug_file.and(warn_file);

	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into())
		.add_directive("sqlx=warn".parse().unwrap());


	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_thread_ids(true)
		.with_thread_names(true)
		.with_line_number(true)
		.with_writer(stdout.and(all_files))
		.with_ansi(false)
		.init();

	match line.trim() {
		"1" => {}
		"2" => { hooks = false; }
		"3" => { add_webhook().await? }
		"4" => { remove_webhook()? }
		"5" => {
			hooks = false;
			test_hook().await?;
		}
		_ => {
			tracing::error!("Bad options - aborting");
			exit(1);
		}
	};

	warn!("Started core loop");
	fetch_loop(hooks).await;
	Ok(())
}