use std::fs;

use log::*;
use serenity::http::Http;

use crate::json_to_structs::recent::*;
use crate::json_to_structs::webhooks::*;

pub async fn handle_wt_news_webhook(content: String, index: usize) {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let mut recent: Root = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	if recent.targets[index].recent_url != content {
		println!("New post found, hooking now");
		warn!("New post found, hooking now");

		recent.targets[index].recent_url = content.clone();
		let write = serde_json::to_string(&recent).unwrap();
		fs::write("assets/recent.json", write).expect("Couldn't write to recent file");
		println!("Written {} to file at index {}", content, index);
		warn!("Written {} to file at index {}", content, index);

		execute_wt_news_webhooks(&content, index).await;
	} else {
		println!("Content was recently fetched and is not new");
		info!("Content was recently fetched and is not new");
	}





	async fn execute_wt_news_webhooks(content: &String, index: usize) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for hook in &webhook_auth.hooks {
			let default_keywords = vec![
				"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
				"fix", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
				"shop", "pass", "season", "operation", "pass", "summer", "2021", "planned", "bonds"
			];

			let filter = &webhook_auth.hooks[index].filter;
			match filter {
				FilterType::Default => for keyword in default_keywords {
					if content.contains(keyword) {
						println!("URL {} matched with default keyword {}", content, keyword);
						warn!("URL {} matched with default keyword {}", content, keyword);
						send_hook(content.to_string(), &hook).await;
						return;
					}
				},
				FilterType::Blacklist => {
					let blacklist = &webhook_auth.hooks[index].keywords;
					if blacklist.is_empty() {
						send_hook(content.to_string(), &hook).await;
						return;
					} else {
						for keyword in blacklist {
							if !content.contains(keyword) {
								println!("No blacklisted items found in {}", content);
								warn!("No blacklisted items found in {}", content);
								send_hook(content.to_string(), &hook).await;
								return;
							}
						}
					}
				}
				FilterType::Whitelist => {
					let whitelist = &webhook_auth.hooks[index].keywords;
					for keyword in whitelist {
						if content.contains(keyword) {
							println!("URL {} matched with whitelisted keyword {}", content, keyword);
							warn!("URL {} matched with whitelisted keyword {}", content, keyword);
							send_hook(content.to_string(), hook).await;
							return;
						}
					}
				}
			}
			async fn send_hook(content: String, hook: &Hooks) {
				let uid = hook.uid;
				let token = &hook.token;

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
					// w.embeds(vec![embed]);
					w.avatar_url("https://cdn.discordapp.com/attachments/866634236232597534/868623209631744000/the_news_broke.png");
					w
				})
					.await
					.unwrap();
			}
			// panics when Enum couldn't be matched ( if this occurs, check discord_token.json for "filter"
			// error!("Enum could not be matched to a value");
			// panic!("Enum could not be matched to a value")
			println!("All WT news hooks are served");
			info!("All WT news hooks are served");
		}
	}
}

pub async fn handle_simple_webhook(content: String, index: usize) {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let mut recent: Root = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	if recent.targets[index].recent_url != content {
		println!("New post found, hooking now");
		warn!("New post found, hooking now");

		recent.targets[index].recent_url = content.clone();
		let write = serde_json::to_string(&recent).unwrap();
		fs::write("assets/recent.json", write).expect("Couldn't write to recent file");
		println!("Written {} to file at index {}", content, index);
		warn!("Written {} to file at index {}", content, index);

		execute_forum_webhooks(&content).await;
	} else {
		println!("Content was recently fetched and is not new");
		info!("Content was recently fetched and is not new");
	}



	async fn execute_forum_webhooks(content: &String) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for hook in webhook_auth.hooks {
			let uid = hook.uid;
			let token = hook.token;

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
				// w.embeds(vec![embed]);
				w.avatar_url("https://cdn.discordapp.com/attachments/866634236232597534/868623209631744000/the_news_broke.png");
				w
			})
				.await
				.unwrap();
		}
		println!("All forum hooks are served");
		info!("All forum hooks are served");
	}
}