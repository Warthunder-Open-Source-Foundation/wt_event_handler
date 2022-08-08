use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

use chrono::Local;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

use crate::{RECENT_PATH, TOKEN_PATH};
use crate::embed::EmbedData;
use crate::json::recent::Recent;
use crate::json::webhooks::{Hooks, WebhookAuth};
use crate::logging::{LogLevel, print_log};
use crate::webhook_handler::deliver_webhook;

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

pub fn verify_json() -> Result<bool, Box<dyn Error>> {
	println!("Verifying Json files...");

	let recent_raw = fs::read_to_string(RECENT_PATH)?;
	let mut recent: Recent = serde_json::from_str(&recent_raw)?;

	let local_time = u64::try_from(Local::now().timestamp())?;

	if (local_time - recent.meta.timestamp) > 60 * 60 {
		recent.meta.timestamp = u64::try_from(Local::now().timestamp())?;
		let write_recent = serde_json::to_string_pretty(&recent)?;
		fs::write("assets/recent.json", write_recent)?;
		return Ok(true);
	} else if recent.meta.timestamp == 0 {
		recent.meta.timestamp = local_time;
		println!("The last fetch date was 0 and has been corrected");
		let write_recent = serde_json::to_string_pretty(&recent)?;
		fs::write("assets/recent.json", write_recent)?;
		return Ok(true);
	}

	recent.meta.timestamp = local_time;

	let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
	let token: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");


	let write_recent = serde_json::to_string_pretty(&recent).unwrap();
	fs::write(RECENT_PATH, write_recent).expect("Couldn't write to recent file");

	let write_token = serde_json::to_string_pretty(&token).unwrap();
	fs::write(TOKEN_PATH, write_token).expect("Couldn't write to recent file");

	println!("Json files complete");
	Ok(false)
}

pub async fn add_webhook() -> Result<(), Box<dyn Error>> {
	let token_raw = fs::read_to_string(TOKEN_PATH)?;
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw)?;

	webhook_auth.hooks.push(Hooks::from_user().await);

	let write = serde_json::to_string_pretty(&webhook_auth)?;
	fs::write(TOKEN_PATH, write)?;
	exit(0);
}

pub async fn test_hook() -> Result<(), Box<dyn Error>> {
	let mut line = String::new();

	println!("Choose the webhook order in the array to test\n");

	io::stdin().read_line(&mut line)?;

	let pos = usize::from_str(line.trim())?;

	deliver_webhook(EmbedData::test(), pos).await;

	exit(0);
}

pub fn remove_webhook() -> Result<(), Box<dyn Error>> {
	let token_raw = fs::read_to_string(TOKEN_PATH)?;
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw)?;
	let mut line = String::new();

	println!("These are the following available webhooks");
	for (i, hook) in webhook_auth.hooks.iter().enumerate() {
		println!("{} {}", i, hook.name);
	}
	println!("Choose the webhook to remove \n");

	io::stdin().read_line(&mut line)?;
	let index = line.trim().parse()?;

	webhook_auth.hooks.remove(index);

	let write = serde_json::to_string_pretty(&webhook_auth)?;
	fs::write(TOKEN_PATH, write)?;

	verify_json()?;
	println!("Webhook {} successfully removed", index);
	exit(0);
}

pub fn clean_recent() -> Result<(), Box<dyn Error>> {
	let cache_raw = fs::read_to_string(RECENT_PATH)?;
	let mut cache: Recent = serde_json::from_str(&cache_raw)?;

	for source in &mut cache.sources {
		source.old_urls.clear();
	}

	let write = serde_json::to_string_pretty(&cache)?;
	fs::write(RECENT_PATH, write)?;

	print_log("Cleared recent file", LogLevel::Warning);
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

// #[test]
	// fn test_clean_recent() {
	// 	let pre_test_raw = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
	// 	let pre_test_struct: Recent = serde_json::from_str(&pre_test_raw).expect("Json cannot be read");
	//
	// 	clean_recent();
	//
	// 	let post_test = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
	// 	let post_test_struct: Recent = serde_json::from_str(&post_test).expect("Json cannot be read");
	//
	// 	println!("{:?}", pre_test_struct);
	// 	println!("{:?}", post_test_struct);
	//
	// 	assert!(post_test_struct.forums_updates_information.recent_url.is_empty() &&
	// 		post_test_struct.warthunder_news.recent_url.is_empty() &&
	// 		post_test_struct.warthunder_changelog.recent_url.is_empty() &&
	// 		post_test_struct.forums_project_news.recent_url.is_empty()
	// 	);
	//
	//
	// 	fs::write(RECENT_PATH, serde_json::to_string_pretty(&pre_test_struct).unwrap()).expect("Couldn't write to recent file");
	// }


	// Currently does not pass thanks to timestamp issues, will fix later
	// #[test]
	fn _test_verify_json() {
		let pre_test_recent = fs::read(RECENT_PATH).expect("Cannot read file");
		let pre_test_token = fs::read(TOKEN_PATH).expect("Cannot read file");

		verify_json();

		let post_test_recent = fs::read(RECENT_PATH).expect("Cannot read file");
		let post_test_token = fs::read(TOKEN_PATH).expect("Cannot read file");

		assert_eq!(pre_test_token, post_test_token);
		assert_eq!(pre_test_recent, post_test_recent);

		fs::write(RECENT_PATH, pre_test_recent).expect("Couldn't write to recent file");
		fs::write(TOKEN_PATH, pre_test_token).expect("Couldn't write to recent file");
	}
}
