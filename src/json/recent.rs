use std::collections::HashSet;
use std::fs;
use tokio::sync::RwLock;

use tracing::{info, warn};

use crate::RECENT_PATH;
use crate::scrapers::html_processing::scrape_links;
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct Sources {
	pub sources: Vec<Source>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Source {
	pub name: String,
	pub domain: String,
	pub scrape_type: ScrapeType,
	#[serde(skip_serializing, skip_deserializing)]
	pub tracked_urls: RwLock<HashSet<String>>,
}

impl Source {
	pub async fn is_new(&self, value: &str) -> bool {
		!self.tracked_urls.read().await.get(value).is_some()
	}
	pub async fn store_recent(&mut self, value: &impl ToString) {
		self.tracked_urls.write().await.insert(value.to_string());
	}
}

impl Sources {
	/// Reads source URLs from drive and pre-loads URLs
	pub async fn build_from_drive() -> Self {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let recent: Self = serde_json::from_str::<Self>(&cache_raw_recent).expect("Json cannot be read").pre_populate_urls().await;
		recent
	}
	async fn pre_populate_urls(self) -> Self {
		warn!("Pre-fetching URLs");
		let mut new = self;
		for source in &mut new.sources {
			match scrape_links(source).await {
				Ok(news_urls) => {
					for news_url in news_urls {
						source.store_recent(&news_url).await;
					}
				}
				Err(e) => {
					panic!("Failed to prefetch source-data: {}", e);
				}
			}
		}

		new
	}
}
