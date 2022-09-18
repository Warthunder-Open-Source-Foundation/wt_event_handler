use std::fmt::{Display, Formatter};
use std::time::Duration;

use reqwest::Client;
use scraper::Html;
use tracing::info;

use crate::error::NewsError;
use crate::scrapers::scraper_resources::html_util::{ElemUtil, format_selector, HtmlUtil};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
/// Defines the types of pages where news come from
pub enum ScrapeType {
	Forum,
	Main,
	Changelog,
}

impl ScrapeType {
	// Used for API calls or similar
	pub fn infer_from_url(url: &str) -> Self {
		if url.contains("warthunder.com") {
			if url.contains("changelog") {
				Self::Changelog
			} else {
				Self::Main
			}
		} else {
			Self::Forum
		}
	}
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

pub async fn request_html(url: &str) -> Result<Html, NewsError> {
	info!("Fetching data from {}", &url);

	let client = Client::builder()
		.timeout(Duration::from_secs(5))
		.build()?;
	let raw_html = client.get(url).send().await?;
	let text = raw_html.text().await?;
	Ok(Html::parse_document(text.as_str()))
}

pub fn get_listed_links(scrape_type: ScrapeType, html: &Html) -> Result<Vec<String>, NewsError> {
	return match scrape_type {
		ScrapeType::Changelog | ScrapeType::Main => {
			let sel_text = if scrape_type == ScrapeType::Main {
				// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
				"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div.showcase__item"
			} else {
				// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
				"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(3) > div > section > div > div.showcase__content-wrapper > div.showcase__item"
			};
			let sel = format_selector(sel_text)?;

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				if let Ok(url) = item.select_first("a", &scrape_type.to_string())?.select_attribute("href", &scrape_type.to_string()) {
					res.push(url.clone());
				}
			}
			Ok(res)
		}
		ScrapeType::Forum => {
			static SEL_TEXT: &str = "body > main > div > div > div > div:nth-child(2) > div > ol > li";
			let sel = format_selector(SEL_TEXT)?;

			let lower_url_test = "div > h4 > div > a";
			let lower_url = format_selector(lower_url_test)?;

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				if let Some(url_elem) = item.select(&lower_url).next() {
					if let Some(url) = url_elem.value().attr("href") {
						res.push(url.to_owned());
					}
				}
			}
			Ok(res)
		}
	};
}

pub fn format_into_final_url(top_url: &str, selection: ScrapeType) -> String {
	match selection {
		ScrapeType::Main | ScrapeType::Changelog => {
			format!("https://warthunder.com{}", top_url)
		}
		ScrapeType::Forum => {
			top_url.to_owned()
		}
	}
}