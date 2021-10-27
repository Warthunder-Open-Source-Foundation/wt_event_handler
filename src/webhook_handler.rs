use std::fs;

use log::{error, warn};
use serenity::http::Http;

use crate::json::recent::Value;
use crate::json::webhooks::{FilterType, Hooks, WebhookAuth};
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::TOKEN_PATH;

const DEFAULT_KEYWORDS: [&str; 27] = [
	"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
	"fix", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
	"shop", "pass", "season", "operation", "pass", "summer", "2021", "planned", "bonds", "issues", "technical", "servers",
];

impl Value {
	pub async fn handle_webhook(&self, content: &str, is_filtered: bool, scrape_type: ScrapeType) {
		let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for (i, hook) in webhook_auth.hooks.iter().enumerate() {
			if is_filtered {
				if matches_filter(content, hook, scrape_type) {
					deliver_webhooks(content, i).await;
				}
			} else {
				deliver_webhooks(content, i).await;
			}
		}
	}
}

fn matches_filter(content: &str, hook: &Hooks, scrape_type: ScrapeType) -> bool {
	match scrape_type {
		ScrapeType::Main => {
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
					print_log(&format!("URL {} matched with default main keyword {}", content, keyword));
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in main default list", content));
			false
		}
		FilterType::Blacklist => {
			let blacklist = &hook.main_keywords;
			if blacklist.is_empty() {
				print_log(&format!("URL {} matched empty blacklist for main", content));
				return true;
			}
			for keyword in blacklist {
				if content.contains(keyword) {
					print_log(&format!("URL {} found in blacklist for main", content));
					return false;
				}
			}
			print_log(&format!("{} is not in main blacklist", content));
			true
		}
		FilterType::Whitelist => {
			let whitelist = &hook.main_keywords;
			for keyword in whitelist {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with whitelisted keyword {} from main list", content, keyword));
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in main list", content));
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
					print_log(&format!("URL {} matched with default forum keyword {}", content, keyword));
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in forum default list", content));
			false
		}
		FilterType::Blacklist => {
			let blacklist = &hook.forum_keywords;
			println!("{:?}", blacklist);
			if blacklist.is_empty() {
				print_log(&format!("URL {} matched empty blacklist for forum", content));
				return true;
			}
			for keyword in blacklist {
				if content.contains(keyword) {
					print_log(&format!("URL {} found in blacklist for forum", content));
					return false;
				}
			}
			print_log(&format!("{} is not in forum blacklist", content));
			true
		}
		FilterType::Whitelist => {
			let whitelist = &hook.forum_keywords;
			for keyword in whitelist {
				if content.contains(keyword) {
					print_log(&format!("URL {} matched with whitelisted keyword {} from forum list", content, keyword));
					return true;
				}
			}
			print_log(&format!("URL {} did not match any whitelist in forum list", content));
			false
		}
	}
}

//Finally sends the webhook to the servers
async fn deliver_webhooks(content: &str, pos: usize) {
	let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
	let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

	let uid = webhook_auth.hooks[pos].uid;
	let token = &webhook_auth.hooks[pos].token;

	let my_http_client = Http::new_with_token(token);

	let webhook = match my_http_client.get_webhook_with_token(uid, token).await {
		Err(why) => {
			println!("{}", why);
			error!("{}", why);
			panic!("")
		}
		Ok(hook) => hook,
	};

	webhook.execute(my_http_client, false, |w| {
		w.content(&format!("[{a}]()", a = content));
		w.username("The WT news bot");
		w.avatar_url("https://cdn.discordapp.com/attachments/866634236232597534/868623209631744000/the_news_broke.png");
		w
	}).await.unwrap();
}

fn print_log(input: &str) {
	println!("{}", input);
	warn!("{}", input);
}

// mod tests {
// 	#[allow(unused_imports)]
// 	use crate::json::webhooks::FilterType::{Blacklist, Whitelist};
//
// 	#[allow(unused_imports)]
// 	use super::*;
//
// 	#[test]
// 	fn test_filter_default_pass() {
// 		assert_eq!(match_filter("pass", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: FilterType::default(),
// 			forum_filter: FilterType::default(),
// 			main_keywords: vec![],
// 			forum_keywords: vec![],
// 		}).unwrap(), "pass")
// 	}
//
// 	#[test]
// 	#[should_panic]
// 	fn test_filter_default_no_match() {
// 		match_filter("xyz", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: FilterType::default(),
// 			forum_filter: FilterType::default(),
// 			main_keywords: vec![],
// 			forum_keywords: vec![],
// 		}).unwrap();
// 	}
//
// 	#[test]
// 	fn test_filter_whitelist_match() {
// 		assert_eq!(match_filter("C", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: Whitelist,
// 			forum_filter: Blacklist,
// 			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
// 		}).unwrap(), "C");
// 	}
//
// 	#[test]
// 	#[should_panic]
// 	fn test_filter_whitelist_miss() {
// 		match_filter("E", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: Whitelist,
// 			forum_filter: Blacklist,
// 			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 			forum_keywords: vec!["W".to_owned(), "X".to_owned(), "Y".to_owned(), "Z".to_owned()],
// 		}).unwrap();
// 	}
//
// 	#[test]
// 	#[should_panic]
// 	fn test_filter_blacklist_match() {
// 		match_filter("C", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: Blacklist,
// 			forum_filter: Blacklist,
// 			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 		}).unwrap();
// 	}
//
// 	#[test]
// 	fn test_filter_blacklist_miss() {
// 		match_filter("E", &Hooks {
// 			name: "".to_string(),
// 			token: "".to_string(),
// 			uid: 0,
// 			main_filter: Blacklist,
// 			forum_filter: Blacklist,
// 			main_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 			forum_keywords: vec!["A".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()],
// 		}).unwrap();
// 	}
// }
