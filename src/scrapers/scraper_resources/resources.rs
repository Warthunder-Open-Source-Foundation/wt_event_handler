use std::process::exit;

use log::{error, info};
use reqwest::get;
use scraper::{ElementRef, Html, Selector};

use crate::json::recent::{format_selector, Channel};

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
	println!("Fetching data from {}", &url);
	info!("Fetching data from {}", &url);

	let html;
	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
			return Some(html);
		}
		return None;
	}
	None
}

pub fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
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
					println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
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
					println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
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