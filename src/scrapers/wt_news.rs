use std::{fs};

use log::*;
use reqwest::get;
use scraper::{Html, Selector};
use crate::json_to_structs::recent::*;

pub async fn html_processor_wt_news(index: usize) -> String {
	let cache_raw_recent = fs::read_to_string("recent.json").expect("Cannot read file");
	let recent: Root = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");

	let url = &recent.targets[index].domain;

	println!("Fetching data from {}", url);

	if get(url).await.is_err() {
		println!("Cannot fetch data");
		error!("Cannot fetch data from {}", url);
		return "fetch_failed".to_string()
	}

	let html = Html::parse_document(&get(url)
		.await
		.unwrap()
		.text()
		.await
		.unwrap());

	let top_url_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap();

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = &*format!("https://warthunder.com{}", top_url.value().attr("href").unwrap());
		top_url.to_string()
	} else {
		"fetch_failed".to_string()
	}
}