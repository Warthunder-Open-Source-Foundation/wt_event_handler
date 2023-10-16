use std::collections::HashMap;

use tracing::{error, warn};

use crate::api::database::Database;
use crate::error::NewsError;
use crate::scrapers::html_processing::scrape_links;
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct Sources {
	pub sources: Vec<Source>,
}

pub type NewsArticle = HashMap<String, i64>;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Source {
	pub name: String,
	pub domain: String,
	pub id: u8,
	pub scrape_type: ScrapeType,
	#[serde(skip_serializing, skip_deserializing)]
	pub(crate) tracked_urls: NewsArticle,
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
	pub async fn build(db: &Database) -> Result<Self, NewsError> {
		let recent = Self::new()
			.pre_populate_urls(db.clone())
			.await?;

		Ok(recent)
	}

	async fn pre_populate_urls(self, db: Database) -> Result<Self, NewsError> {
		warn!("Pre-fetching URLs");
		let mut new = self;
		for source in &mut new.sources {
			match scrape_links(source).await {
				Ok(news_urls) => {
					for news_url in &news_urls {
						source.store_recent([&news_url]);
						db.store_recent([&news_url], source.id).await.unwrap();
					}
				}
				Err(e) => {
					error!("Failed to prefetch source-data: {}", e);
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
			let mut removed = false;
			for source in &mut self.sources {
				eprintln!("Removed {}", to_remove.to_string());
				if source.tracked_urls.remove(&to_remove.to_string()).is_some() {
					removed = true;
				}
			}
			if !removed {
				eprintln!("Failed to remove URL: {}", to_remove.to_string());
			}
		}
	}
}
