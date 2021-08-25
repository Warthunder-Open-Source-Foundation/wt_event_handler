use std::option::Option::Some;

use scraper::Selector;

use crate::scrapers::scraper_resources::resources::{fetch_failed, get_local, pinned, request_html};

pub async fn html_processor_wt_news() -> Option<String> {
	let recent = get_local();

	let url = &recent.warthunder_news.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed();
	}


	let selectors = [
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap(),
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(2) > a").unwrap()
	];
	let pin = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > div.widget__pin").unwrap();

	let mut top_url: Vec<String> = vec![String::new(), String::new()];

	//Assigns 1st URL to 1st post
	if let Some(x) = html.select(&selectors[0]).next() {
		top_url[0] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};

	//Assigns 2nd URL top 2nd post
	if let Some(x) = html.select(&selectors[1]).next() {
		top_url[1] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};


	if let Some(pin_url) = html.select(&pin).next() {
		//Checks if a "widget__pin" exists on 1st post
		let pin_url = pin_url.value().attr("class").unwrap();
		if pin_url == "widget__pin" {
			//verifies if latest post is already known, and then cho
			return Some(pinned(&recent.warthunder_news.recent_url, &top_url).clone());
		}
		return Some(top_url[0].clone());
	}
	return fetch_failed();
}