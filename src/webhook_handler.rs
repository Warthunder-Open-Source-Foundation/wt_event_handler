use std::fs;

use log::*;
use serenity::http::Http;

use crate::json_to_structs::recent::*;
use crate::json_to_structs::webhooks::*;

//Receives latest content and index in recent array (for WT news)
pub async fn handle_wt_news_webhook(content: String, index: usize) {
	if handle_recent(content.clone(), index) {
		execute_wt_news_webhooks(&content).await
	}

	async fn execute_wt_news_webhooks(content: &String) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for (i, hook) in webhook_auth.hooks.iter().enumerate() { ;
			let default_keywords = vec![
				"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
				"fix", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
				"shop", "pass", "season", "operation", "pass", "summer", "2021", "planned", "bonds"
			];
			let filter = &hook.filter;

			match filter {
				FilterType::Default => for keyword in default_keywords {
					if content.contains(keyword) {
						println!("URL {} matched with default keyword {}", content, keyword);
						warn!("URL {} matched with default keyword {}", content, keyword);
						deliver_webhooks(&content, i).await;
					}
				},
				FilterType::Blacklist => {
					let blacklist = &webhook_auth.hooks[i].keywords;
					if blacklist.is_empty() {
						deliver_webhooks(&content, i).await;
					} else {
						for keyword in blacklist {
							if !content.contains(keyword) {
								println!("No blacklisted items found in {}", content);
								warn!("No blacklisted items found in {}", content);
								deliver_webhooks(&content, i).await;
							}
						}
					}
				}
				FilterType::Whitelist => {
					let whitelist = &webhook_auth.hooks[i].keywords;
					for keyword in whitelist {
						if content.contains(keyword) {
							println!("URL {} matched with whitelisted keyword {}", content, keyword);
							warn!("URL {} matched with whitelisted keyword {}", content, keyword);
							deliver_webhooks(&content, i).await;
						}
					}
				}
			}
			// panics when Enum couldn't be matched ( if this occurs, check discord_token.json for "filter"
		}
		println!("All forum hooks are served");
		info!("All forum hooks are served");
		write_latest(&content, index);
	}
}

//Receives latest content and index in recent array
pub async fn handle_simple_webhook(content: String, index: usize) {
	if handle_recent(content.clone(), index) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for i in 0..webhook_auth.hooks.len() {
			deliver_webhooks(&content, i).await;
		}
		println!("All forum hooks are served");
		info!("All forum hooks are served");
		write_latest(&content, index);
	}
}

//Finally sends the webhook to the servers
async fn deliver_webhooks(content: &String, pos: usize) {
	let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

	let uid = webhook_auth.hooks[pos].uid;
	let token = &webhook_auth.hooks[pos].token;

	let my_http_client = Http::new_with_token(&token);

	let webhook = match my_http_client.get_webhook_with_token(uid, &token).await {
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
	})
		.await
		.unwrap();
}

//Checks if the given content is new, and sends webhook if yes
fn handle_recent(content: String, index: usize) -> bool {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let mut recent: Recent = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	if recent.targets[index].recent_url[0] != content {
		println!("New post found, hooking now");
		warn!("New post found, hooking now");
		true
	} else {
		println!("Content was recently fetched and is not new");
		info!("Content was recently fetched and is not new");
		false
	}
}

fn write_latest(content: &String, index: usize) {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let mut recent: Recent = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	recent.targets[index].recent_url.insert(0, content.clone());
	let write = serde_json::to_string_pretty(&recent).unwrap();
	fs::write("assets/recent.json", write).expect("Couldn't write to recent file");
	println!("Written {} to file at index {}", content, index);
	warn!("Written {} to file at index {}", content, index);
}