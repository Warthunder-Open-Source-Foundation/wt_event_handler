use std::error::Error;

use crate::embed::EmbedData;
use crate::error::error_webhook;
use crate::json::recent::Channel;
use crate::scrapers::scrape_meta::scrape_meta;
use crate::scrapers::scraper_resources::resources::{format_into_final_url, get_listed_links, request_html, ScrapeType};

/// Returns all embeds for new news posts
pub async fn html_processor(channel: &mut Channel) -> Result<Vec<EmbedData>, Box<dyn Error>> {
	let scrape_type = channel.scrape_type;

	let mut links = scrape_links(channel).await?;

	// Removes already known URLs
	links.retain(|link| channel.is_new(link, false));

	let mut final_embeds = vec![];
	for link in links {
		let embed = get_embed_data(&link, scrape_type).await?;
		final_embeds.push(embed);
	}

	Ok(final_embeds)
}

/// Returns embed-ready information per URL source
pub async fn get_embed_data(url: &str, scrape_type: ScrapeType) -> Result<EmbedData, Box<dyn Error>> {
	let post_html = request_html(url).await?;
	Ok(match scrape_meta(&post_html, scrape_type, url) {
		Ok(ok) => ok,
		Err(e) => {
			error_webhook(&e, true).await;
			EmbedData::fail_over(url, scrape_type)
		}
	})
}

/// Returns all URLs per channel
pub async fn scrape_links(channel: &Channel) -> Result<Vec<String>, Box<dyn Error>> {
	let html = request_html(&channel.domain).await?;

	let mut urls = get_listed_links(channel.scrape_type, &html)?;
	for url in &mut urls {
		*url = format_into_final_url(url, channel.scrape_type);
	}
	Ok(urls)
}