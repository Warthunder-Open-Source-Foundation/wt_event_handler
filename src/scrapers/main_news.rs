use std::option::Option::Some;
use std::process::exit;

use log::error;
use scraper::Selector;

use crate::json_to_structs::recent::{format_selector, Value};
use crate::scrapers::scraper_resources::resources::{fetch_failed, request_html};

pub async fn html_processor_warthunderdotcom(recent_value: &Value) -> Option<String> {
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
		pin = format_selector(&recent_value, "pin", post);

		if let Some(_top_url) = html.select(&pin).next() {
			post += 1;
		} else {
			break;
		}
		if post > 20 {
			println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
			exit(-1);
		}
	}

	let top_url_selector = format_selector(&recent_value, "selector", post);

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = format!("https://warthunder.com{}", top_url.value().attr("href").unwrap());
		Some(top_url)
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}