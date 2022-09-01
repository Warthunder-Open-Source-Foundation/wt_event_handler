use std::collections::HashSet;
use std::fs;
use serde_json::Value;
use tokio::sync::RwLock;

use tracing::{warn};

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
	tracked_urls: RwLock<HashSet<String>>,
	#[serde(skip_serializing, skip_deserializing)]
	pub json: RwLock<String>,
}

impl Source {
	pub async fn is_new(&self, value: &str) -> bool {
		!self.tracked_urls.read().await.get(value).is_some()
	}

	pub async fn store_recent<I>(&self, value: I) -> Result<(), Box<dyn std::error::Error>>
		where I: IntoIterator,
			  I::Item: ToString
	{
		let iter = value.into_iter().map(|s| s.to_string());
		{
			self.tracked_urls.write().await.extend(iter);
		}
		self.update_json().await
	}

	async fn update_json(&self) -> Result<(), Box<dyn std::error::Error>> {
		let json_value = match serde_json::to_value(&self)? {
			Value::Object(mut map) => {
				let tracked_urls = self.tracked_urls.read().await;
				map.insert("tracked_urls".to_owned(), serde_json::to_value(&*tracked_urls)?);

				serde_json::to_value(map)?
			}
			_ => { unreachable!() } // unreachable because we know it can't happen. We just passed in a struct ("Object") to get the value
		};
		let json = serde_json::to_string(&json_value)?;

		{
			*self.json.write().await = json;
		}

		Ok(())
	}
}

impl Sources {
	/// Reads source URLs from drive and pre-loads URLs
	pub async fn build_from_drive() -> Result<Self, Box<dyn std::error::Error>> {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let recent: Self = serde_json::from_str::<Self>(&cache_raw_recent)
			.expect("Json cannot be read")
			.pre_populate_urls()
			.await?;

		Ok(recent)
	}

	async fn pre_populate_urls(self) -> Result<Self, Box<dyn std::error::Error>> {
		warn!("Pre-fetching URLs");
		let mut new = self;
		for source in &mut new.sources {
			match scrape_links(source).await {
				Ok(news_urls) => {
					for news_url in news_urls {
						source.store_recent(&[&news_url]).await?;
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
	pub async fn debug_remove_tracked_urls<I>(&self, to_remove_urls: I)
		where I: IntoIterator,
			  I::Item: ToString
	{
		for to_remove in to_remove_urls.into_iter() {
			for source in &self.sources {
				source.tracked_urls.write().await.remove(&to_remove.to_string());
			}
		}
	}
}
