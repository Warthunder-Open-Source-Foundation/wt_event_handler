use scraper::{Html, Selector};

use crate::embed::EmbedData;
use crate::scrapers::scraper_resources::resources::ScrapeType;

pub fn scrape_meta(html: &Html, scrape_type: ScrapeType, post_url: &str) -> EmbedData {
	let (title, img_url, preview_text) = match scrape_type {
		ScrapeType::Forum => {
			scrape_forum(html)
		}
		ScrapeType::Main => {
			scrape_main(html)
		}
		ScrapeType::Changelog => {
			scrape_changelog(html)
		}
	};

	EmbedData::new(&title, post_url, &img_url, &preview_text, scrape_type)
}

fn scrape_forum(html: &Html) -> (String, String, String) {
	(
		html.select(&Selector::parse("head>meta:nth-child(5)").unwrap()).next().unwrap().value().attr("content").unwrap_or("").to_string(),
		"".to_string(),
		html.select(&Selector::parse("head>meta:nth-child(8)").unwrap()).next().unwrap().value().attr("content").unwrap_or("").to_string()
	)
}

fn scrape_main(html: &Html) -> (String, String, String) {
	(
		html.select(&Selector::parse("head>meta:nth-child(13)").unwrap()).next().unwrap().value().attr("content").unwrap_or("").to_string(),
		{
			scrape_news_image(html)
		},
		html.select(&Selector::parse("p").unwrap()).next().unwrap().inner_html()
	)
}

fn scrape_changelog(html: &Html) -> (String, String, String) {
	(
		html.select(&Selector::parse("head>meta:nth-child(13)").unwrap()).next().unwrap().value().attr("content").unwrap_or("").to_string(),
		{
			scrape_news_image(html)
		},
		"The current provided changelog reflects the major changes within the game as part of this Update. Some updates, additions and fixes may not be listed in the provided notes. War Thunder is constantly improving and specific fixes may be implemented without the client being updated.".to_string()
	)
}

fn scrape_news_image(html: &Html) -> String {
	let mut actual = "".to_owned();
	for item in html.select(&Selector::parse("meta, img").unwrap()) {
		if let Some(proper_image) = item.value().attr("content") {
			if proper_image.contains("https://warthunder.com/upload/image//!") && item.value().attr("name") != Some("twitter:image") {
				actual = proper_image.to_owned();
				break;
			}
		}

		if let Some(proper_image) = item.value().attr("src") {
			actual = proper_image.to_owned();
			break;
		}
	}
	actual
}

#[cfg(test)]
mod tests {
	use crate::scrapers::scrape_meta::scrape_meta;
	use crate::scrapers::scraper_resources::resources::{request_html, ScrapeType};

	#[tokio::test]
	async fn test_embed_data_main() {
		// let url = "https://warthunder.com/en/news/7598-development-lav-ad-revolving-firepower-en";
		let url = "https://warthunder.com/en/news/7594-development-pre-order-ztz96a-prototype-china-en";
		let html = request_html(url).await.unwrap();

		eprintln!("{:#?}", scrape_meta(&html, ScrapeType::Main, url.to_owned()));
	}

	#[tokio::test]
	async fn test_embed_data_changelog() {
		let url = "https://warthunder.com/en/game/changelog/current/1352";
		let html = request_html(url).await.unwrap();

		eprintln!("{:#?}", scrape_meta(&html, ScrapeType::Changelog, url.to_owned()));
	}
}