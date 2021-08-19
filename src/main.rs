use std::{io, time};
use std::option::Option::Some;
use std::thread::sleep;

use log::*;
use rand::Rng;

use crate::json_to_structs::recent::Recent;
use crate::menu_options::{add_webhook, init_log, remove_webhook, verify_json};
use crate::scrapers::forum_news::html_processor_wt_forums;
use crate::scrapers::wt_changelog::html_processor_wt_changelog;
use crate::scrapers::wt_news::html_processor_wt_news;

mod webhook_handler;
mod scrapers;
mod json_to_structs;
mod menu_options;


#[tokio::main]
async fn main() {
	let mut line = String::new();
	let mut hooks = true;
	let mut json_verification = true;

	println!("Please select a start profile: \n 1. Regular initialization \n 2. Initialize without self-tests \n 3. Boot without sending hooks \n 4. Add new webhook-client \n 5. Remove a webhook");
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
		_ => {
			println!("No option specified")
		}
	}

	if json_verification {
		verify_json();
	}

	init_log();
	println!("Started client");
	info!("Started client");

	let mut recent_data = Recent::read_latest();

	loop {
		if let Some(wt_news_content) = html_processor_wt_news().await {
			if recent_data.warthunder_news.is_outdated(&wt_news_content) && hooks {
				recent_data.warthunder_news.handle_wt_news_webhook(&wt_news_content).await;
				recent_data.append_latest_warthunder_news(&wt_news_content);
				println!("All wt news hooks are served");
				info!("All wt news hooks are served");
				continue
			}
		};

		if let Some(wt_changelog) = html_processor_wt_changelog().await {
			if recent_data.warthunder_changelog.is_outdated(&wt_changelog) && hooks {
				recent_data.warthunder_changelog.handle_simple_webhook(&wt_changelog).await;
				recent_data.append_latest_warthunder_changelog(&wt_changelog);
				println!("All wt changelog hooks are served");
				info!("All wt changelog hooks are served");
				continue
			}
		};

		if let Some(forum_news) = html_processor_wt_forums().await {
			if recent_data.forums.is_outdated(&forum_news) && hooks {
				recent_data.forums.handle_simple_webhook(&forum_news).await;
				recent_data.append_latest_warthunder_forums(&forum_news);
				println!("All forum hooks are served");
				info!("All forum hooks are served");
				continue
			}
		};

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		info!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}