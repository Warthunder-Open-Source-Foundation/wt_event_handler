use std::error::Error;

use crate::embed::EmbedData;
use crate::error::{error_webhook, NewsError};
use crate::json::recent::{Channel, format_selector};
use crate::scrapers::scrape_meta::scrape_meta;
use crate::scrapers::scraper_resources::resources::{format_result, pin_loop, RecentHtmlTarget, request_html};

pub async fn html_processor(recent_value: &Channel) -> Result<EmbedData, Box<dyn Error>> {
	let url = &recent_value.domain;
	let scrape_type = recent_value.scrape_type;

	let html = request_html(url).await?;
	let mut post: u32 = 1;

	post = pin_loop(post, &html, recent_value, scrape_type);

	let top_url_selector = format_selector(recent_value, &RecentHtmlTarget::Post, post);

	let top_url = html.select(&top_url_selector).next().ok_or_else(|| NewsError::NoUrlOnPost(url.clone(), format!("{html:#?}")))?;
	let post_url = format_result(top_url, scrape_type);

	let post_html = request_html(&post_url).await?;

	let finished = match scrape_meta(&post_html, scrape_type, &post_url) {
		Ok(ok) => ok,
		Err(e) => {
			error_webhook(&e, true).await;
			EmbedData::fail_over(&post_url, scrape_type)
		}
	};

	Ok(finished)
}