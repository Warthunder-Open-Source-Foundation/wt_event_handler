use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::time::Duration;

use log::info;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::json::recent::{Channel, format_selector};
use crate::{LogLevel, print_log};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ScrapeType {
	Forum,
	Main,
	Changelog,
}

impl Display for ScrapeType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ScrapeType::Forum => {
				write!(f, "Forum news")
			}
			ScrapeType::Main => {
				write!(f, "News")
			}
			ScrapeType::Changelog => {
				write!(f, "Changelog")
			}
		}
	}
}

pub enum RecentHtmlTarget {
	Pin,
	Post,
}

pub async fn request_html(url: &str) -> Result<Html, Box<dyn Error>> {
	print_log(&format!("Fetching data from {}", &url), LogLevel::Info);

	let client = Client::builder()
		.timeout(Duration::from_secs(1))
		.build()?;
	let raw_html = client.get(url).send().await?;
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