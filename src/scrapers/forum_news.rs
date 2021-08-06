use std::{fs};

use log::*;
use reqwest::get;
use scraper::{Html, Selector};
use crate::json_to_structs::recent::*;

pub async fn html_processor_wt_forums() -> Option<String> {

	let cache_raw = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let cache: Recent = serde_json::from_str(&cache_raw).expect("Json cannot be read");

	let url = &cache.forums.domain;

	println!("Fetching data from {}", url);
	info!("Fetching data from {}", url);

	let html;

	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
		}else {
			return None
		}
	}else {
		return None
	}

	let top_url_selector = Selector::parse("body > main > div > div > div > div:nth-child(2) > div > ol > li:nth-child(2) > div > h4 > div > a").unwrap();

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = top_url.value().attr("href").unwrap();
		Some(top_url.to_string())
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	}
}