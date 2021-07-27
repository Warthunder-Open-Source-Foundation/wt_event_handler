use std::{fs};

use log::*;
use reqwest::get;
use scraper::{Html, Selector};
use crate::json_to_structs::recent::*;
use crate::json_to_structs::webhooks::*;

pub async fn html_processor_wt_news(index: usize) -> String {
	let cache_raw_recent = fs::read_to_string("recent.json").expect("Cannot read file");
	let recent: Root = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	let cache_raw_recent = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let webhook: WebhookAuth = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	let url = &recent.targets[index].domain;

	println!("Fetching data from {}", url);

	if get(url).await.is_err() {
		println!("Cannot fetch data");
		error!("Cannot fetch data from {}", url);
		return "fetch_failed".to_string()
	}

	let html = Html::parse_document(&get(url)
		.await
		.unwrap()
		.text()
		.await
		.unwrap());

	let top_url_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap();

	let top_url = html.select(&top_url_selector)
		.next()
		.unwrap()
		.value()
		.attr("href")
		.unwrap();


	let default_keywords = vec![
		"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
		"fixed", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
		"shop", "pass", "season", "operation", "pass", "summer", "2021"
	];
	let top_url = &*format!("https://warthunder.com{}", top_url);

	match &webhook.hooks[index].filter {
		FilterType::Default => for keyword in default_keywords {
			if top_url.contains(keyword) {
				println!("URL {} matched with default keyword {}", top_url, keyword);
				warn!("URL {} matched with default keyword {}", top_url, keyword);
				return (top_url).parse().unwrap();
			}
		},
		FilterType::Blacklist => {
			let blacklist = &webhook.hooks[index].keywords;
			for keyword in blacklist {
				if !top_url.contains(keyword) {
					println!("No blacklisted items found in {}", top_url);
					warn!("No blacklisted items found in {}", top_url);
					return (top_url).parse().unwrap();
				}
			}
		},
		FilterType::Whitelist => {
			let whitelist = &webhook.hooks[index].keywords;
			for keyword in whitelist {
				if top_url.contains(keyword) {
					println!("URL {} matched with whitelisted keyword {}", top_url, keyword);
					warn!("URL {} matched with whitelisted keyword {}", top_url, keyword);
					return (top_url).parse().unwrap();
				}
			}
		}
	}

	let result = &recent.targets[index].recent_url;
	return result.to_string();
}