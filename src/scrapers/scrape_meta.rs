use scraper::{Html, Selector};
use crate::embed::EmbedData;
use crate::scrapers::scraper_resources::resources::ScrapeType;

pub fn scrape_meta(html: &Html, scrape_type: ScrapeType, post_url: String) -> EmbedData {
	let (title, img_url, preview_text) = match scrape_type {
		ScrapeType::Forum => {
			(
				html.select(&Selector::parse("head>meta:nth-child(5)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
				"".to_string(),
				html.select(&Selector::parse("head>meta:nth-child(8)").unwrap_or(Selector::parse("meta").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string()
			)
		}
		ScrapeType::Main => {
			(
				html.select(&Selector::parse("head>meta:nth-child(13)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
				{
					let mut actual= "".to_owned();
					let _ = html.select(&Selector::parse("img").unwrap_or(Selector::parse("html").unwrap())).for_each(|item|{
						if let Some(proper_image) = item.value().attr("src") {
							actual = proper_image.to_owned();
						}
					});
					actual
				},
				html.select(&Selector::parse("p").unwrap()).next().unwrap().inner_html()
			)
		}
		ScrapeType::Changelog => {
			(
				html.select(&Selector::parse("head>meta:nth-child(13)").unwrap_or(Selector::parse("html").unwrap())).next().unwrap().value().attr("content").unwrap_or("").to_string(),
				{
					let mut actual= "".to_owned();
					let _ = html.select(&Selector::parse("img").unwrap_or(Selector::parse("html").unwrap())).for_each(|item|{
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

	EmbedData::new(&title, &post_url, &img_url, &preview_text, scrape_type)
}