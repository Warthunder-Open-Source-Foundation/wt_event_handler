use std::option::Option::Some;
use std::process::exit;

use log::error;
use scraper::Selector;

use crate::scrapers::scraper_resources::resources::{fetch_failed, request_html};
use crate::json_to_structs::recent::{RecentValue, format_selector};

pub async fn html_processor_wt_forums(recent_value: &RecentValue) -> Option<String> {
	let url = &recent_value.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	let mut pin: Selector;

	loop {
		pin = format_selector(&recent_value, "pin", &post);
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

	let top_url_selector = format_selector(&recent_value, "selector", &post);

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = top_url.value().attr("href").unwrap();
		Some(top_url.to_string())
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}