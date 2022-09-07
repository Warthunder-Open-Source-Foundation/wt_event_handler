use std::collections::{HashMap};
use std::fs;
use serde_json::Value;
use tokio::sync::RwLock;

use tracing::{error, warn};
use crate::api::database::Database;
use crate::error::NewsError;

use crate::RECENT_PATH;
use crate::scrapers::html_processing::scrape_links;
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct Sources {
	pub sources: Vec<Source>,
	#[serde(skip_serializing, skip_deserializing)]
	latest_news: RwLock<Vec<(String, String, i64)>>,
}

pub type NewsArticle = HashMap<String, i64>;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Source {
	pub name: String,
	pub domain: String,
	pub scrape_type: ScrapeType,
	#[serde(skip_serializing, skip_deserializing)]
	tracked_urls: NewsArticle,
}

impl Source {
	pub fn is_new(&self, value: &str) -> bool {
		self.tracked_urls.get(value).is_none()
	}

	pub fn store_recent<I>(&mut self, value: I)
		where I: IntoIterator,
			  I::Item: ToString
	{
		let iter = value.into_iter().map(|s| (s.to_string(), chrono::Utc::now().timestamp()));
		{
			self.tracked_urls.extend(iter);
		}
	}


}

impl Sources {
	/// Reads source URLs from drive and pre-loads URLs
	pub async fn build_from_drive(db: &Database) -> Result<Self, NewsError> {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let mut recent: Self = serde_json::from_str::<Self>(&cache_raw_recent)
			.expect("Json cannot be read")
			.pre_populate_urls(db.clone())
			.await?;

		recent.update_latest().await;

		Ok(recent)
	}

	async fn pre_populate_urls(self, db: Database) -> Result<Self, NewsError> {
		warn!("Pre-fetching URLs");
		let mut new = self;
		for source in &mut new.sources {
			match scrape_links(source).await {
				Ok(news_urls) => {
					for news_url in &news_urls {
						source.store_recent(&[&news_url]);
							db.store_recent(&[&news_url], &source.name).await;
					}
				}
				Err(e) => {
					panic!("Failed to prefetch source-data: {}", e);
				}
			}
		}

		Ok(new)
	}

	/// Removes any URL from all tracked URLs
	pub fn debug_remove_tracked_urls<I>(&mut self, to_remove_urls: I)
		where I: IntoIterator,
			  I::Item: ToString
	{
		for to_remove in to_remove_urls {
			for source in &mut self.sources {
				source.tracked_urls.remove(&to_remove.to_string());
			}
		}
	}

	// Source-name, URL, timestamp
	pub async fn update_latest(&self) {
		let mut latest = vec![];
		for source in &self.sources {
			let mut latest_item = ("No news yet".to_owned(), i64::MIN);
			for item in &source.tracked_urls {
				if *item.1 > latest_item.1 {
					latest_item = (item.0.clone(), *item.1);
				}
			}
			latest.push((source.name.clone(), latest_item.0, latest_item.1));
		}
		*self.latest_news.write().await = latest;
	}

	// Source-name, URL, timestamp
	pub async fn get_latest(&self) -> Vec<(String, String, i64)> {
		// self.latest_news.read().await.iter().map(|(source, url, timestamp)|(source.as_str(), url.as_str(), *timestamp)).collect::<Vec<(&'a str, &'a str, i64)>>()
		self.latest_news.read().await.clone()
	}
}
