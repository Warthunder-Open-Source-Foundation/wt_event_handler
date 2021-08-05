use std::{fs, io, time};
use std::option::Option::Some;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;

use chrono::offset::Local;
use log::*;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use rand::Rng;

use crate::json_to_structs::recent::Recent;
use crate::json_to_structs::webhooks::*;
use crate::scrapers::forum_news::html_processor_wt_forums;
use crate::scrapers::wt_changelog::html_processor_wt_changelog;
use crate::scrapers::wt_news::html_processor_wt_news;

mod webhook_handler;
mod scrapers;
mod json_to_structs;


#[tokio::main]
async fn main() {
	let mut line = String::new();
	let mut no_hooks = false;
	let mut no_json_verification = true;

	println!("Please select a start profile: \n 1. Regular initialization \n 2. Initialize without self-tests \n 3. Boot without sending hooks \n 4. Add new webhook-client \n 5. Remove a webhook");
	io::stdin()
		.read_line(&mut line)
		.expect("failed to read from stdin");

	let trimmed = line.trim();

	match trimmed {
		"1" => {}
		"2" => {
			no_json_verification = false;
		}
		"3" => {
			no_hooks = true;
		}
		"4" => {
			add_webhook().await;
		}
		"5" => {
			remove_webhook();
		}
		_ => {
			println!("No option specified")
		}
	}

	if no_json_verification {
		verify_json();
	}

	init_log();
	println!("Started client");
	info!("Started client");

	let mut recent_data = Recent::read_latest();

	loop {
		if let Some(wt_news_content) = html_processor_wt_news().await {
			if recent_data.warthunder_news.is_outdated(&wt_news_content) && !no_hooks {
				recent_data.warthunder_news.handle_wt_news_webhook(&wt_news_content).await;
				recent_data.append_latest_warthunder_news(&wt_news_content);
				println!("All wt news hooks are served");
				info!("All wt news hooks are served");
			}
		};

		if let Some(wt_changelog) = html_processor_wt_changelog().await {
			if recent_data.warthunder_changelog.is_outdated(&wt_changelog) && !no_hooks {
				recent_data.warthunder_changelog.handle_simple_webhook(&wt_changelog).await;
				recent_data.append_latest_warthunder_changelog(&wt_changelog);
				println!("All wt changelog hooks are served");
				info!("All wt changelog hooks are served");
			}
		};

		if let Some(forum_news) = html_processor_wt_forums().await {
			if recent_data.forums.is_outdated(&forum_news) && !no_hooks {
				recent_data.forums.handle_simple_webhook(&forum_news).await;
				recent_data.append_latest_warthunder_forums(&forum_news);
				println!("All forum hooks are served");
				info!("All forum hooks are served");
			}
		};

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		info!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}

fn init_log() {
	if Path::new("log/latest.log").exists() {
		let now = Local::now().format("%Y_%m_%d_%H-%M-%S").to_string();
		fs::rename("log/latest.log", format!("log/old/{}.log", now)).expect("Could not rename latest log file");
	}

	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{l} {d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
		.build("log/latest.log").unwrap();

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.build(Root::builder()
			.appender("logfile")
			.build(LevelFilter::Info)).unwrap();

	log4rs::init_config(config).unwrap();
}

fn verify_json() {
	println!("Verifying Json files...");
	let recent_raw = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let mut recent: Recent = serde_json::from_str(&recent_raw).expect("Json cannot be read");
	//Just for removing warning
	recent.warthunder_changelog.recent_url.pop();
	let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let mut entry: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");
	//Just for removing warning
	entry.hooks.pop();
	println!("Json files complete");
}

async fn add_webhook() {
	let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

	webhook_auth.hooks.push(Hooks::from_user().await);

	let write = serde_json::to_string_pretty(&webhook_auth).unwrap();
	fs::write("assets/discord_token.json", write).expect("Couldn't write to recent file");
	exit(0);
}

fn remove_webhook() {
	let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");
	let mut line = String::new();

	println!("These are the following available webhooks");
	for (i, hook) in webhook_auth.hooks.iter().enumerate() {
		println!("{} {}", i, hook.name);
	}
	println!("Choose the webhook to remove \n");

	io::stdin().read_line(&mut line).unwrap();
	let index = line.trim().parse().unwrap();

	webhook_auth.hooks.remove(index);

	let write = serde_json::to_string_pretty(&webhook_auth).unwrap();
	fs::write("assets/discord_token.json", write).expect("Couldn't write to recent file");

	verify_json();
	println!("Webhook {} successfully removed", index);
	exit(0);
}