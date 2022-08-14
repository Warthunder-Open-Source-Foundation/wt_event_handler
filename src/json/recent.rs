use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::logging::{LogLevel, print_log};
use crate::RECENT_PATH;
use crate::scrapers::html_processing::{get_embed_data, html_processor, scrape_links};
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Sources {
	pub sources: Vec<Channel>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
	pub name: String,
	pub domain: String,
	pub scrape_type: ScrapeType,
	#[serde(skip_serializing, skip_deserializing)]
	pub tracked_urls: HashSet<String>,
}

impl Channel {
	pub fn is_new(&self, value: &str, output: bool) -> bool {
		if self.tracked_urls.get(&value.to_owned()).is_some() {
			if output {
				print_log("Content was recently fetched and is not new", LogLevel::Info);
			}
			false
		} else {
			if output {
				print_log("New post found, hooking now", LogLevel::Warning);
			}
			true
		}
	}
	pub fn store_recent(&mut self, value: impl ToString) {
		self.tracked_urls.insert(value.to_string());
	}
}

impl Sources {
	pub async fn read_latest() -> Self {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let mut recent: Self = serde_json::from_str::<Self>(&cache_raw_recent).expect("Json cannot be read").pre_populate_urls().await;
		recent
	}
	async fn pre_populate_urls(&self) -> Self {
		print_log("Pre-fetching URLs", LogLevel::Info);
		let mut new = self.clone();
		for mut source in &mut new.sources {
			match scrape_links(&source).await {
				Ok(news_urls) => {
					for news_url in news_urls {
						source.store_recent(news_url);
					}
				}
				Err(e) => {
					panic!("Failed to prefetch source-data: {}", e);
				}
			}
		}
		new.clone()
	}
}
