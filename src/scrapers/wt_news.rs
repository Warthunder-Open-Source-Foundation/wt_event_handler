use std::fs;
use std::option::Option::Some;

use log::*;
use reqwest::get;
use scraper::{Html, Selector};

use crate::json_to_structs::recent::*;

pub async fn html_processor_wt_news() -> Option<String> {
	let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
	let recent: Recent = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	let url = &recent.warthunder_news.domain;

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
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap(),
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(2) > a").unwrap()
	];
	let pin = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > div.widget__pin").unwrap();

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
			return Some(pinned(recent, &top_url).clone());
		} else {
			return Some(top_url[0].clone());
		}
	} else {
		return fetch_failed();
	}

	fn pinned(recent: Recent, top_url: &Vec<String>) -> &String {
		let recents = &recent.warthunder_news.recent_url;
		if !recents.contains(&top_url[0]) {
			&top_url[0]
		} else {
			&top_url[1]
		}
	}

	fn fetch_failed() -> Option<String> {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	}
}