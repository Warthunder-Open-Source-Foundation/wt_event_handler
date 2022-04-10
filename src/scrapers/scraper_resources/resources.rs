use std::error::Error;
use std::process::exit;

use log::{info};
use reqwest::get;
use scraper::{ElementRef, Html, Selector};
use crate::embed::EmbedData;

use crate::json::recent::{format_selector, Channel};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ScrapeType {
	Forum,
	Main,
	Changelog,
}

impl ToString for ScrapeType {
	fn to_string(&self) -> String {
		match self {
			ScrapeType::Forum => {
				"Forum news".to_owned()
			}
			ScrapeType::Main => {
				"News".to_owned()
			}
			ScrapeType::Changelog => {
				"Changelog".to_owned()
			}
		}
	}
}

pub enum RecentHtmlTarget {
	Pin,
	Post,
}

pub async fn request_html(url: &str) -> Result<Html, Box<dyn Error>> {
	println!("{} Fetching data from {}", chrono::Local::now(), &url);
	info!("{} Fetching data from {}", chrono::Local::now(), &url);

	let raw_html = get(url).await?;
	let text = raw_html.text().await?;
	Ok(Html::parse_document(text.as_str()))
}

pub fn pin_loop(mut post: u32, html: &Html, recent_value: &Channel, selection: ScrapeType) -> u32 {
	let mut pin: Selector;

	match selection {
		ScrapeType::Main | ScrapeType::Changelog => {
			loop {
				pin = format_selector(recent_value, &RecentHtmlTarget::Pin, post);
				if html.select(&pin).next().is_some() {
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
		ScrapeType::Main | ScrapeType::Changelog => {
			format!("https://warthunder.com{}", top_url.value().attr("href").unwrap())
		}
		ScrapeType::Forum => {
			top_url.value().attr("href").unwrap().to_string()
		}
	};
}