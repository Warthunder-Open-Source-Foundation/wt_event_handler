use std::option::Option::Some;
use std::process::exit;

use log::error;
use scraper::Selector;

use crate::scrapers::scraper_resources::resources::{fetch_failed, get_local, request_html};

pub async fn html_processor_wt_forums_project_news() -> Option<String> {
	let recent = get_local();

	let url = &recent.forums_project_news.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	let mut pin: Selector;

	loop {
		pin = Selector::parse(&*format!("body > main > div > div > div > div:nth-child(2) > div > ol > li:nth-child({})", post)).unwrap();

		if let Some(top_url) = html.select(&pin).next() {
			let is_pinned = top_url.value().attr("class").unwrap().contains("pinned");
			if !is_pinned {
				break;
			}
			post += 1;
		}
		if post > 20 {
			println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
			exit(-1);
		}
	}

	let top_url_selector = Selector::parse(&*format!("body > main > div > div > div > div:nth-child(2) > div > ol > li:nth-child({}) > div > h4 > div > a", post)).unwrap();

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = top_url.value().attr("href").unwrap();
		Some(top_url.to_string())
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}