use std::io;
use std::process::exit;

use log::info;

use crate::fetch_loop::fetch_loop;
use crate::menu_options::{add_webhook, clean_recent, init_log, remove_webhook, verify_json};

mod webhook_handler;
mod scrapers;
mod json_to_structs;
mod menu_options;
mod fetch_loop;

const RECENT_PATH: &str = "assets/recent.json";
const TOKEN_PATH: &str = "assets/discord_token.json";

#[tokio::main]
async fn main() {
	let mut line = String::new();
	let mut hooks = true;
	let mut json_verification = true;
	let mut json_prefetch_required = false;

	println!("Please select a start profile: \n \
	1. Regular initialization \n \
	2. Initialize without self-tests \n \
	3. Boot without sending hooks \n \
	4. Add new webhook-client \n \
	5. Remove a webhook \n \
	6. Clean and reload recent file");
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	match line.trim() {
		"1" => {}
		"2" => {
			json_verification = false;
		}
		"3" => {
			hooks = false;
		}
		"4" => {
			add_webhook().await;
		}
		"5" => {
			remove_webhook();
		}
		"6" => {
			hooks = false;
			json_verification = false;
			clean_recent();
		}
		_ => {
			println!("No option specified");
			exit(1);
		}
	}

	if json_verification {
		json_prefetch_required = verify_json();
	}

	if json_prefetch_required {
		clean_recent();
		println!("Json prefetched and cleaned successfully");
		exit(0);
	}

	init_log();
	println!("Started client");
	info!("Started client");

	fetch_loop(hooks).await;
}