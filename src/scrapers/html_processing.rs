use std::error::Error;

use crate::embed::EmbedData;
use crate::error::error_webhook;
use crate::json::recent::Channel;
use crate::scrapers::scrape_meta::scrape_meta;
use crate::scrapers::scraper_resources::resources::{format_result, get_listed_links, request_html};

pub async fn html_processor(recent_value: &Channel) -> Result<EmbedData, Box<dyn Error>> {
	let url = &recent_value.domain;
	let scrape_type = recent_value.scrape_type;

	let html = request_html(url).await?;

	let links = get_listed_links(scrape_type, &html)?;

	dbg!(&links);

	let post_url = format_result(&links[0].0, scrape_type);

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