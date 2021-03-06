use std::fs;

use log::{error, info, warn};
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::utils::Color;

use crate::embed::EmbedData;
use crate::fetch_loop::STATS;
use crate::json::recent::Channel;
use crate::json::webhooks::{FilterType, Hooks, WebhookAuth};
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::statistics::Incr;
use crate::TOKEN_PATH;

const DEFAULT_KEYWORDS: [&str; 30] = [
	"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
	"fix", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
	"shop", "pass", "season", "operation", "pass", "summer", "2021", "planned", "bonds",
	"issues", "technical", "servers", "christmas", "market", "camouflages"
];

impl Channel {
	pub async fn handle_webhook(&self, content: &EmbedData, is_filtered: bool, scrape_type: ScrapeType) {
		let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for (i, hook) in webhook_auth.hooks.iter().enumerate() {
			if is_filtered {
				if match_filter(&content.url, hook, scrape_type) {
					deliver_webhook(content.clone(), i).await;
				}
			} else {
				deliver_webhook(content.clone(), i).await;
			}
			STATS.lock().await.increment(Incr::PostCounter);
		}
	}
}

fn match_filter(content: &str, hook: &Hooks, scrape_type: ScrapeType) -> bool {
	match scrape_type {
		ScrapeType::Main | ScrapeType::Changelog => {
			filter_main(content, hook)
		}
		ScrapeType::Forum => {
			filter_forum(content, hook)
		}
	}
}

fn filter_main(content: &str, hook: &Hooks) -> bool {
	let main_filter = &hook.main_filter;

	match main_filter {
		FilterType::Default => {
			for keyword in DEFAULT_KEYWORDS {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with default main keyword {}", content, keyword), 1);
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in main default list", content), 1);
			false
		}
		FilterType::Blacklist => {
			let blacklist = &hook.main_keywords;
			if blacklist.is_empty() {
				print_log(&format!("URL {} matched empty blacklist for main", content), 1);
				return true;
			}
			for keyword in blacklist {
				if content.contains(keyword) {
					print_log(&format!("URL {} found in blacklist for main", content), 1);
					return false;
				}
			}
			print_log(&format!("{} is not in main blacklist", content), 1);
			true
		}
		FilterType::Whitelist => {
			let whitelist = &hook.main_keywords;
			for keyword in whitelist {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with whitelisted keyword {} from main list", content, keyword), 1);
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in main list", content), 1);
			false
		}
	}
}

fn filter_forum(content: &str, hook: &Hooks) -> bool {
	let forum_filter = &hook.forum_filter;

	match forum_filter {
		FilterType::Default => {
			for keyword in DEFAULT_KEYWORDS {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with default forum keyword {}", content, keyword), 1);
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in forum default list", content), 1);
			false
		}
		FilterType::Blacklist => {
			let blacklist = &hook.forum_keywords;
			if blacklist.is_empty() {
				print_log(&format!("URL {} matched empty blacklist for forum", content), 1);
				return true;
			}
			for keyword in blacklist {
				if content.contains(keyword) {
					print_log(&format!("URL {} found in blacklist for forum", content), 1);
					return false;
				}
			}
			print_log(&format!("{} is not in forum blacklist", content), 1);
			true
		}
		FilterType::Whitelist => {
			let whitelist = &hook.forum_keywords;
			for keyword in whitelist {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with whitelisted keyword {} from forum list", content, keyword), 1);
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in forum list", content), 1);
			false
		}
	}
}

//Finally sends the webhook to the servers
pub async fn deliver_webhook(content: EmbedData, pos: usize) {
	let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
	let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

	let uid = webhook_auth.hooks[pos].uid;
	let token = &webhook_auth.hooks[pos].token;

	let my_http_client = Http::new(token);

	let webhook = match my_http_client.get_webhook_with_token(uid, token).await {
		Err(why) => {
			print_log(&format!("{why}"), 0);
			std::panic::panic_any(why)
		}
		Ok(hook) => hook,
	};

	let embed = Embed::fake(|e| {
		e.title(content.scrape_type.to_string())
		 .color(Color::from_rgb(116, 16, 210))
		 .field(&content.title, &content.preview_text, false)
		 .description(format!("Fetched on: <t:{}>", chrono::offset::Local::now().timestamp()))
		 .thumbnail("https://avatars.githubusercontent.com/u/97326911?s=40&v=4")
		 .image(&content.img_url)
		 .url(&content.url)
		 .footer(|f| {
			 f.icon_url("https://warthunder.com/i/favicons/mstile-70x70.png").text("Report bugs/issues: FlareFlo????#2800")
		 })
	});

	webhook.execute(my_http_client, false, |w| {
		w.content(&format!("[{}]()", &content.url));
		w.embeds(vec![embed]);
		w
	}).await.unwrap();
	print_log(&format!("Posted webhook for {}", webhook_auth.hooks[pos].name), 1);
}

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

// Tests  -----------------------------------------------------------------------

mod tests {
	#[allow(unused_imports)]
	use crate::json::webhooks::FilterType::{Blacklist, Whitelist};

	#[allow(unused_imports)]
	use super::*;

	// Main tests -------------------------------------------------------------------
	#[test]
	fn main_test_filter_default_pass() {
		assert_eq!(match_filter("pass", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: FilterType::default(),
			forum_filter: FilterType::default(),
			main_keywords: vec![],
			forum_keywords: vec![],
		}, ScrapeType::Main), true)
	}

	#[test]
	fn main_test_filter_default_no_match() {
		assert_eq!(match_filter("xyz", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: FilterType::default(),
			forum_filter: FilterType::default(),
			main_keywords: vec![],
			forum_keywords: vec![],
		}, ScrapeType::Main), false);
	}

	#[test]
	fn main_test_filter_whitelist_match() {
		assert_eq!(match_filter("C", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Whitelist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
		}, ScrapeType::Main), true);
	}

	#[test]
	#[should_panic]
	fn main_test_filter_whitelist_miss() {
		assert_eq!(match_filter("E", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Whitelist,
			forum_filter: Whitelist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
		}, ScrapeType::Main), true);
	}

	#[test]
	#[should_panic]
	fn main_test_filter_blacklist_match() {
		assert_eq!(match_filter("C", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Blacklist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
		}, ScrapeType::Main), true);
	}

	#[test]
	fn main_test_filter_blacklist_miss() {
		assert_eq!(match_filter("E", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Blacklist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
		}, ScrapeType::Main), true);
	}

	// forum tests ------------------------------------------------------------------

	#[test]
	fn forum_test_filter_default_pass() {
		assert_eq!(match_filter("pass", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: FilterType::default(),
			forum_filter: FilterType::default(),
			main_keywords: vec![],
			forum_keywords: vec![],
		}, ScrapeType::Forum), true)
	}

	#[test]
	fn forum_test_filter_default_no_match() {
		assert_eq!(match_filter("xyz", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: FilterType::default(),
			forum_filter: FilterType::default(),
			main_keywords: vec![],
			forum_keywords: vec![],
		}, ScrapeType::Forum), false);
	}

	#[test]
	fn forum_test_filter_whitelist_match() {
		assert_eq!(match_filter("C", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Whitelist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
		}, ScrapeType::Forum), true);
	}

	#[test]
	fn forum_test_filter_whitelist_miss() {
		assert_eq!(match_filter("E", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Whitelist,
			forum_filter: Whitelist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
		}, ScrapeType::Forum), false);
	}

	#[test]
	fn forum_test_filter_blacklist_match() {
		assert_eq!(match_filter("C", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Blacklist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
		}, ScrapeType::Forum), false);
	}

	#[test]
	fn forum_test_filter_blacklist_miss() {
		match_filter("E", &Hooks {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: Blacklist,
			forum_filter: Blacklist,
			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
		}, ScrapeType::Forum);
	}
}
