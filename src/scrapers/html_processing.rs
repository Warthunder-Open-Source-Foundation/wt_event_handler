use crate::embed::EmbedData;
use crate::error::{error_webhook, NewsError};
use crate::json::sources::Source;
use crate::scrapers::scrape_meta::scrape_meta;
use crate::scrapers::scraper_resources::resources::{format_into_final_url, get_listed_links, request_html, ScrapeType};

/// Returns all embeds for new news posts
pub async fn html_processor(source: &Source) -> Result<Vec<EmbedData>, NewsError> {
	let scrape_type = source.scrape_type;

	let mut links = scrape_links(source).await?;

	// Removes already known URLs
	let mut positions = vec![];
	for (position, link) in links.iter().enumerate() {
		if !source.is_new(link).await {
			positions.push(position);
		}
	}

	positions.reverse();
	for position in positions  {
		links.remove(position);
	}


	let mut final_embeds = vec![];
	for link in links {
		let embed = get_embed_data(&link, scrape_type).await?;
		final_embeds.push(embed);
	}

	Ok(final_embeds)
}

/// Returns embed-ready information per URL source
pub async fn get_embed_data(url: &str, scrape_type: ScrapeType) -> Result<EmbedData, NewsError> {
	let post_html = request_html(url).await?;
	Ok(match scrape_meta(&post_html, scrape_type, url) {
		Ok(ok) => ok,
		Err(e) => {
			error_webhook(&e, "",true).await;
			EmbedData::fail_over(url, scrape_type)
		}
	})
}

/// Returns all URLs per channel
pub async fn scrape_links(channel: &Source) -> Result<Vec<String>, NewsError> {
	let html = request_html(&channel.domain).await?;

	let mut urls = get_listed_links(channel.scrape_type, &html)?;
	for url in &mut urls {
		*url = format_into_final_url(url, channel.scrape_type);
	}
	Ok(urls)
}