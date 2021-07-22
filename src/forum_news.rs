use std::{fs, mem};

use reqwest::get;
use scraper::{Html, Selector};

pub async fn html_processor_wt_forums(index: usize) -> String {
	#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
	pub struct Root {
		pub targets: Vec<Target>,
	}

	#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
	pub struct Target {
		pub name: String,
		pub recent_url: String,
		pub domain: String,
	}

	let cache_raw = fs::read_to_string("recent.json").expect("Cannot read file");
	let mut cache: Root = serde_json::from_str(&cache_raw).expect("Json cannot be read");

	let url = &cache.targets[index].domain;

	println!("Fetching data from {}", url);

	if get(url).await.is_err() {
		println!("Cannot fetch data");
		return "fetch_failed".to_string()
	}

	let html = Html::parse_document(&get(url)
		.await
		.unwrap()
		.text()
		.await
		.unwrap());
	println!("Fetched data with size of {} bytes", mem::size_of_val(&html));

	let top_url_selector = Selector::parse("body > main > div > div > div > div:nth-child(2) > div > ol > li:nth-child(2) > div > h4 > div > a").unwrap();

	let top_url = html.select(&top_url_selector)
		.next()
		.unwrap()
		.value()
		.attr("href")
		.unwrap();

	return (top_url).parse().unwrap();
}