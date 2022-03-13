use std::option::Option::Some;
use scraper::Selector;
use crate::embed::EmbedData;

use crate::json::recent::{format_selector, Channel};
use crate::scrapers::scraper_resources::resources::{fetch_failed, format_result, pin_loop, RecentHtmlTarget, request_html, ScrapeType};

pub async fn html_processor(recent_value: &Channel, scrape_type: ScrapeType) -> Option<EmbedData> {
	let url = &recent_value.domain;

	let html;
	if let Some(value) = request_html(url).await {
		html = value;
	} else {
		return fetch_failed();
	}

	let mut post: u32 = 1;

	post = pin_loop(post, &html, recent_value, scrape_type);

	let top_url_selector = format_selector(recent_value, &RecentHtmlTarget::Post, post);
	if let Some(top_url) = html.select(&top_url_selector).next() {
		let post_url = format_result(top_url, scrape_type);

		let post_html;
		if let Some(value) = request_html(&post_url).await {
			post_html = value;
		} else {
			return fetch_failed();
		}

		let (title, img_url, preview_text) = match scrape_type {
			ScrapeType::Forum => {
				(
					post_html.select(&Selector::parse("head>meta:nth-child(5)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
					"".to_string(),
					post_html.select(&Selector::parse("head>meta:nth-child(8)").unwrap_or(Selector::parse("meta").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string()
				)
			}
			ScrapeType::Main => {
				(
					post_html.select(&Selector::parse("head>meta:nth-child(13)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
					{
						let mut actual= "".to_owned();
						let _ = post_html.select(&Selector::parse("img").unwrap_or(Selector::parse("html").unwrap())).for_each(|item|{
							if let Some(proper_image) = item.value().attr("src") {
								actual = proper_image.to_owned();
							}
						});
						actual
					},
					post_html.select(&Selector::parse("p").unwrap()).next().unwrap().inner_html()
				)
			}
			ScrapeType::Changelog => {
				(
					post_html.select(&Selector::parse("head>meta:nth-child(13)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
					{
						let mut actual= "".to_owned();
						let _ = post_html.select(&Selector::parse("img").unwrap_or(Selector::parse("html").unwrap())).for_each(|item|{
							if let Some(proper_image) = item.value().attr("src") {
								actual = proper_image.to_owned();
							}
						});
						actual
					},
					"The current provided changelog reflects the major changes within the game as part of this Update. Some updates, additions and fixes may not be listed in the provided notes. War Thunder is constantly improving and specific fixes may be implemented without the client being updated.".to_string()
				)
			}
		};

		let embed_data = EmbedData::new(&title, &post_url, &img_url, &preview_text, scrape_type);
		return Some(embed_data);
	}

	fetch_failed()
}