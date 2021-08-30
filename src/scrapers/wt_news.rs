use std::option::Option::Some;
use std::process::exit;

use log::error;
use scraper::Selector;

use crate::scrapers::scraper_resources::resources::{fetch_failed, get_local, request_html};

pub async fn html_processor_wt_news() -> Option<String> {
	let recent = get_local();

	let url = &recent.warthunder_news.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	let mut pin: Selector;

	loop {
		pin = Selector::parse(&*format!("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child({}) > div.widget__pin", post)).unwrap();

		if let Some(top_url) = html.select(&pin).next() {
			post += 1;
		} else {
			break;
		}
		if post > 20 {
			println!("Maximum pinned-post limit exceeded, aborting due to failure in finding unpinned post!");
			exit(-1);
		}
	}

	let top_url_selector = Selector::parse(&*format!("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child({}) > a.widget__link", post)).unwrap();

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = format!("https://warthunder.com{}", top_url.value().attr("href").unwrap());
		Some(top_url)
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}