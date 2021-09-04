use std::option::Option::Some;

use log::error;

use crate::json_to_structs::recent::{format_selector, Value};
use crate::scrapers::scraper_resources::resources::{fetch_failed, format_result, find_unpinned_post, request_html, ScrapeType, RecentHtmlTarget};

pub async fn html_processor(recent_value: &Value, scrape_type: ScrapeType) -> Option<String> {
	let url = &recent_value.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post_enumerator: u32 = 1;

	post_enumerator = find_unpinned_post(post_enumerator, &html, &recent_value, scrape_type);

	let top_url_selector = format_selector(&recent_value, &RecentHtmlTarget::Post, post_enumerator);
	if let Some(top_url) = html.select(&top_url_selector).next() {
		return Some(format_result(top_url, scrape_type));
	}
	println!("Fetch failed");
	error!("Fetch failed");
	return None;
}