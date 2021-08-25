use std::fs;
use std::option::Option::Some;

use log::{error, info};
use reqwest::get;
use scraper::{Html, Selector};

use crate::json_to_structs::recent::Recent;
use crate::scrapers::scraper_resources::resources::{get_local, request_html, fetch_failed, pinned};

pub async fn html_processor_wt_news() -> Option<String> {
	let recent = get_local();

	let url = &recent.warthunder_news.domain;

	let html;
	if let Some(value) = request_html(&url).await{
		html = value;
	}else{
		return fetch_failed()
	}


	let selectors = [
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap(),
		Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(2) > a").unwrap()
	];
	let pin = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > div.widget__pin").unwrap();

	let mut top_url: Vec<String> = vec!["".to_string(), "".to_string()];

	if let Some(x) = html.select(&selectors[0]).next() {
		top_url[0] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};
	if let Some(x) = html.select(&selectors[1]).next() {
		top_url[1] = (&*format!("https://warthunder.com{}", x.value().attr("href").unwrap())).parse().unwrap();
	} else {
		return fetch_failed();
	};

	if let Some(pin_url) = html.select(&pin).next() {
		let pin_url = pin_url.value().attr("class").unwrap();
		if pin_url == "widget__pin" {
			return Some(pinned(&recent, &top_url).clone());
		}
		return Some(top_url[0].clone());
	}
	return fetch_failed();
}