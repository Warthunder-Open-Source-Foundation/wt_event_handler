use log::{error, info};
use reqwest::get;
use scraper::{Html, Selector, ElementRef};
use crate::json_to_structs::recent::{format_selector, Value};
use std::process::exit;

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

	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			let html = Html::parse_document(text.as_str());
			return Some(html);
		}
		return None;
	}
	return None;
}

pub fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
}

#[cfg(test)]
mod tests {
	use super::*;
}

pub fn find_unpinned_post(mut post_eunmerator: u32, html: &Html, recent_value: &Value, scrape_type: ScrapeType) -> u32 {
	let mut pin: Selector;

	match scrape_type {
		ScrapeType::Main => {
			loop {
				pin = format_selector(&recent_value, &RecentHtmlTarget::Pin, post_eunmerator);
				if let Some(_top_url) = html.select(&pin).next() {
					post_eunmerator += 1;
				} else {
					return post_eunmerator;
				}
				if post_eunmerator > 20 {
					println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
					exit(-1);
				}
			}
		}
		ScrapeType::Forum => {
			loop {
				pin = format_selector(&recent_value, &RecentHtmlTarget::Pin, post_eunmerator);
				if let Some(top_url) = html.select(&pin).next() {
					let is_pinned = top_url.value().attr("class").unwrap().contains("pinned");
					if !is_pinned {
						return post_eunmerator
					}
					post_eunmerator += 1;
				}
				if post_eunmerator > 20 {
					println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
					exit(-1);
				}
			}
		}
	}

}

pub fn format_result(post_element: ElementRef, scrape_type: ScrapeType) -> String{
	return match scrape_type {
		ScrapeType::Main => {
			format!("https://warthunder.com{}", post_element.value().attr("href").unwrap())
		}
		ScrapeType::Forum => {
			post_element.value().attr("href").unwrap().to_string()
		}
	}
}