#![feature(if_let_guard)]
#![feature(once_cell)]
#![allow(clippy::module_name_repetitions)]

use std::{fs, io};
use std::error::Error;
use std::process::exit;

use lazy_static::{initialize, lazy_static};

use crate::fetch_loop::fetch_loop;
use crate::json::webhooks::CrashHook;
use crate::json::webhooks::WebhookAuth;
use crate::menu_options::{add_webhook, clean_recent, init_log, remove_webhook, test_hook, verify_json};
use crate::webhook_handler::print_log;

mod webhook_handler;
mod scrapers;
mod json;
mod menu_options;
mod fetch_loop;
mod embed;
mod error;
mod timeout;
mod statistics;

const RECENT_PATH: &str = "assets/recent.json";
const TOKEN_PATH: &str = "assets/discord_token.json";

pub const HANDLE_RESULT_FN: fn(Result<(), Box<dyn Error>>) = |e: Result<(), Box<dyn Error>>| {
	match e {
		Ok(_) => {}
		Err(e) => {
			print_log(&e.to_string(), 0);
			panic!("{}", e);
		}
	}
};

lazy_static! {
	pub static ref WEBHOOK_AUTH: WebhookAuth = {
		let raw = fs::read("assets/discord_token.json").unwrap();
		let json: WebhookAuth = serde_json::from_slice(&raw).unwrap();
		json
	};
	pub static ref PANIC_INFO: CrashHook = {
		WEBHOOK_AUTH.crash_hook[0].clone()
	};
}

#[tokio::main]
async fn main() {
	// Loads statics
	initialize(&WEBHOOK_AUTH);
	initialize(&PANIC_INFO);

	let mut line = String::new();
	let mut hooks = true;
	let mut json_verification = true;
	let mut write_files = true;

	println!("Please select a start profile:\n\
	1. Regular initialization\n\
	2. Initialize without self-tests\n\
	3. Boot without sending hooks\n\
	4. Add new webhook-client\n\
	5. Remove a webhook\n\
	6. Clean and reload recent file\n\
	7. Test webhook client\n\
	0. Debug, does not modify local files");
	io::stdin().read_line(&mut line).expect("failed to read from stdin");

	match line.trim() {
		"0" => { write_files = false }
		"1" => {}
		"2" => { json_verification = false; }
		"3" => { hooks = false; }
		"4" => { HANDLE_RESULT_FN(add_webhook().await) }
		"5" => { HANDLE_RESULT_FN(remove_webhook()) }
		"6" => {
			hooks = false;
			json_verification = false;
			HANDLE_RESULT_FN(clean_recent());
		}
		"7" => {
			hooks = false;
			HANDLE_RESULT_FN(test_hook().await);
		}
		_ => {
			println!("No option specified");
			exit(1);
		}
	}

	if json_verification {
		match verify_json() {
			Ok(result) => {
				if result {
					HANDLE_RESULT_FN(clean_recent());
					print_log("Json prefetched and cleaned successfully", 1);
				}
			}
			Err(e) => {
				print_log(&e.to_string(), 0);
				panic!("{}", e);
			}
		}
	}

	HANDLE_RESULT_FN(init_log());
	print_log("Started client", 1);

	#[allow(clippy::semicolon_if_nothing_returned)]
	fetch_loop(hooks, write_files).await; // For the love of god clippy
}