use log::{error, info};
use reqwest::get;
use scraper::{Html, Selector, ElementRef};
use crate::json_to_structs::recent::format_selector;
use std::process::exit;

pub async fn request_html(url: &str) -> Option<Html> {
	println!("Fetching data from {}", &url);
	info!("Fetching data from {}", &url);

	let html;
	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
			return Some(html);
		}
		return None;
	}
	return None;
}

pub fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
}

pub fn pin_loop_main_news(mut post: u32) -> u32 {
	let mut pin: Selector;

	loop {
		pin = format_selector(&recent_value, "pin", post);

		if let Some(_top_url) = html.select(&pin).next() {
			post += 1;
		} else {
			return post;
		}
		if post > 20 {
			println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
			exit(-1);
		}
	}
}

pub fn format_result(top_url: ElementRef, selection: &str)  -> String{
	match selection {
		"main" => {
			return format!("https://warthunder.com{}", top_url.value().attr("href").unwrap());
		}
		"forum" => {
		return top_url.value().attr("href").unwrap().to_string();
		}
		_ => {
			exit(-1);
		}
	}

}