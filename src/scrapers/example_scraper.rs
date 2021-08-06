use reqwest::get;
use scraper::{Html, Selector};
use std::{fs, mem};

use crate::recent::*;

// TODO change function name
pub async fn html_processor_wt_(index: usize) -> String {
	let cache_raw = fs::read_to_string("recent.json").expect("Cannot read file");
	let mut cache: Root = serde_json::from_str(&cache_raw).expect("Json cannot be read");

	let url = &cache.targets[index].domain;

	println!("Fetching data from {}", url);

	if get(url).await.is_err() {
		println!("Cannot fetch data");
		return "fetch_failed".to_string();
	}

	let html = Html::parse_document(&get(url)
		.await
		.unwrap()
		.text()
		.await
		.unwrap());

	// Doesnt fucking work for some reason, always returns 56. And yes, im probably just stupid.
	println!("Fetched data with size of {} bytes", mem::size_of_val(&html));

	// todo add html selector
	let top_url_selector = Selector::parse("selector").unwrap();

	let top_url = html.select(&top_url_selector)
		.next()
		.unwrap()
		.value()
		.attr("href")
		.unwrap();

	//  TODO add match list if required
	let keywords = vec![];
	let top_url = &*format!("{}", top_url);

	for keyword in keywords {
		if top_url.contains(keyword) {
			println!("URL {} matched with keyword {}", top_url, keyword);
			return (top_url).parse().unwrap();
		}
	}
	return String::from("No match found");
}