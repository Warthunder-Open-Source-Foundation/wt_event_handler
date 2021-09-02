use std::option::Option::Some;
use std::process::exit;

use log::error;
use scraper::{Selector, ElementRef};

use crate::json_to_structs::recent::{format_selector, Value};
use crate::scrapers::scraper_resources::resources::{fetch_failed, request_html, pin_loop_main_news, format_main_news, format_result};

pub async fn html_processor_warthunderdotcom(recent_value: &Value) -> Option<String> {
	let url = &recent_value.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	post = pin_loop_main_news(post);

	let top_url_selector = format_selector(&recent_value, "selector", post);
	return if let Some(top_url) = html.select(&top_url_selector).next() {
		return Some(format_result(top_url, "main"));
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}