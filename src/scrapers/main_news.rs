use std::option::Option::Some;

use log::error;

use crate::json_to_structs::recent::{format_selector, Value};
use crate::scrapers::scraper_resources::resources::{fetch_failed, format_result, pin_loop, request_html, ScrapeType};

pub async fn html_processor(recent_value: &Value, scrape_type: ScrapeType) -> Option<String> {
	let url = &recent_value.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	post = pin_loop(post, &html, &recent_value, scrape_type);

	let top_url_selector = format_selector(&recent_value, "selector", post);
	if let Some(top_url) = html.select(&top_url_selector).next() {
		return Some(format_result(top_url, scrape_type));
	}
	println!("Fetch failed");
	error!("Fetch failed");
	return None;
}