use std::fs;

use log::{error, info};
use reqwest::get;
use scraper::Html;

use crate::json_to_structs::recent::Recent;

pub fn get_local() -> Recent {
	let cache_raw = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let cache: Recent = serde_json::from_str(&cache_raw).expect("Json cannot be read");
	cache
}

pub async fn request_html(url: &str) -> Option<Html> {
	println!("Fetching data from {}", &url);
	info!("Fetching data from {}", &url);

	let html;
	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
			return Some(html)
		} else {
			return None;
		}
	} else {
		return None;
	}
}

pub fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
}

pub fn pinned<'a>(recent: &'a Recent, top_url: &'a [String]) -> &'a String {
	let recents = &recent.warthunder_changelog.recent_url;
	if recents.contains(&top_url[0]) {
		&top_url[1]
	} else {
		&top_url[0]
	}
}