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

pub fn get_listed_links(scrape_type: ScrapeType, html: &Html) -> Vec<String> {
	return match scrape_type {
		ScrapeType::Changelog | ScrapeType::Main => {
			let sel_text = match scrape_type {
				ScrapeType::Main => {
					// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
					"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div > a.widget__link"
				}
				ScrapeType::Changelog => {
					// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
					"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(3) > div > section > div > div.showcase__content-wrapper > div > a.widget__link"
				}
				_ => {
					panic!("Impossible")
				}
			};
			let sel = Selector::parse(sel_text).expect("Selector should be valid as it is static");

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				if let Some(url) = item.value().attr("href") {
					res.push(url.to_owned())
				}
			}
			res
		}
		ScrapeType::Forum => {
			let sel = Selector::parse("body > main > div > div > div > div:nth-child(2) > div > ol > li").expect("Selector should be valid as it is static");

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				let lower_url = Selector::parse("div > h4 > div > a").expect("Selector should be valid as it is static");
				if let Some(url_elem) = item.select(&lower_url).next() {
					if let Some(url) = url_elem.value().attr("href") {
						res.push(url.to_owned())
					}
				}
			}
			res
		}
	}
}

pub fn format_result(top_url: &str, selection: ScrapeType) -> String {
	return match selection {
		ScrapeType::Main | ScrapeType::Changelog => {
			format!("https://warthunder.com{}", top_url)
		}
		ScrapeType::Forum => {
			top_url.to_owned()
		}
	};
}