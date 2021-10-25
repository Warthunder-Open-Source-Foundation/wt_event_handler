use std::process::exit;
use std::time::Duration;

use log::{error, info};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::json_to_structs::recent::{Channel, format_selector};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ScrapeType {
	Forum,
	Main,
}

pub enum RecentHtmlTarget {
	Pin,
	Post,
}

pub async fn request_html(url: &str) -> Option<Html> {
	println!("{} Fetching data from {}", chrono::Local::now(), &url);
	info!("Fetching data from {}", &url);

	let client = Client::builder()
		.connect_timeout(Duration::from_secs(20))
		.timeout(Duration::from_secs(20))
		.build()
		.unwrap();

	let request = client
		.get(url)
		.build()
		.unwrap();

	let html;
	if let Ok(raw_html) = client.execute(request).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
			return Some(html);
		}
		return None;
	}
	None
}

pub fn fetch_failed() -> Option<String> {
	println!("{} Fetch failed", chrono::Local::now());
	error!("Fetch failed");
	None
}

#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
}

pub fn pin_loop(mut post: u32, html: &Html, recent_value: &Channel, selection: ScrapeType) -> u32 {
	let mut pin: Selector;

	match selection {
		ScrapeType::Main => {
			loop {
				pin = format_selector(recent_value, &RecentHtmlTarget::Pin, post);
				if let Some(_top_url) = html.select(&pin).next() {
					post += 1;
				} else {
					return post;
				}
				if post > 20 {
					println!("{} Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!", chrono::Local::now());
					exit(-1);
				}
			}
		}
		ScrapeType::Forum => {
			loop {
				pin = format_selector(recent_value, &RecentHtmlTarget::Pin, post);
				if let Some(top_url) = html.select(&pin).next() {
					let is_pinned = top_url.value().attr("class").unwrap().contains("pinned");
					if !is_pinned {
						return post;
					}
					post += 1;
				}
				if post > 20 {
					println!("{} Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!", chrono::Local::now());
					exit(-1);
				}
			}
		}
	}
}

pub fn format_result(top_url: ElementRef, selection: ScrapeType) -> String {
	return match selection {
		ScrapeType::Main => {
			format!("https://warthunder.com{}", top_url.value().attr("href").unwrap())
		}
		ScrapeType::Forum => {
			top_url.value().attr("href").unwrap().to_string()
		}
	};
}