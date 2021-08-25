use std::fs;

use log::{error, info};
use reqwest::get;
use scraper::{Html, Selector};

use crate::json_to_structs::recent::Recent;

pub async fn html_processor_wt_changelog() -> Option<String> {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let recent: Recent = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	let url = &recent.warthunder_changelog.domain;

	println!("Fetching data from {}", url);
	info!("Fetching data from {}", url);

	let html;

	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
		} else {
			return None;
		}
	} else {
		return None;
	}

	// Too lazy to make !format macro
	let selectors = [
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap(),
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div:nth-child(2) > a").unwrap()
	];
	let pin = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > div.widget__pin").unwrap();

	let mut top_url: Vec<String> = vec!["".to_string(), "".to_string()];

	if let Some(x) = html.select(&selectors[0]).next() {
		top_url[0] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};
	if let Some(x) = html.select(&selectors[1]).next() {
		top_url[1] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};

	if let Some(pin_url) = html.select(&pin).next() {
		let pin_url = pin_url.value().attr("class").unwrap();
		if pin_url == "widget__pin" {
			return Some(pinned(&recent, &top_url).clone());
		}
		return Some(top_url[0].clone());
	}
	return fetch_failed();
}

fn pinned<'a>(recent: &'a Recent, top_url: &'a [String]) -> &'a String {
	let recents = &recent.warthunder_changelog.recent_url;
	if recents.contains(&top_url[0]) {
		&top_url[1]
	} else {
		&top_url[0]
	}
}

fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
}