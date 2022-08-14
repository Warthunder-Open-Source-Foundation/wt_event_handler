use std::error::Error;

use crate::embed::EmbedData;
use crate::error::error_webhook;
use crate::json::recent::{Channel};
use crate::scrapers::scrape_meta::scrape_meta;
use crate::scrapers::scraper_resources::resources::{format_into_final_url, get_listed_links, request_html};

pub async fn html_processor(recent_value: &mut Channel, fetch_inner: bool) -> Result<Vec<EmbedData>, Box<dyn Error>> {
	let url = &recent_value.domain;
	let scrape_type = recent_value.scrape_type;

	let html = request_html(url).await?;

	let mut links: Vec<String> = get_listed_links(scrape_type, &html)?;

	links = links.into_iter().filter(|link|recent_value.is_new(link, false)).collect();

	let mut final_embeds = vec![];
	for link in links {
		let post_url = format_into_final_url(&link, scrape_type);


		let finished = if fetch_inner && recent_value.is_new(&post_url, false) {
			let post_html = request_html(&post_url).await?;
			match scrape_meta(&post_html, scrape_type, &post_url) {
				Ok(ok) => ok,
				Err(e) => {
					error_webhook(&e, true).await;
					EmbedData::fail_over(&post_url, scrape_type)
				}
			}
		} else {
			EmbedData::default()
		};
		final_embeds.push(finished);
	}


	Ok(final_embeds)
}