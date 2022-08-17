use std::error::Error;

use scraper::{Html, Selector};

use crate::embed::EmbedData;
use crate::error::{InputError, NewsError};
use crate::scrapers::scraper_resources::resources::ScrapeType;

/// Collects embed information from page
pub fn scrape_meta(html: &Html, scrape_type: ScrapeType, post_url: &str) -> Result<EmbedData, InputError> {
	let (title, img_url, preview_text) = match scrape_type {
		ScrapeType::Forum => {
			scrape_forum(html)?
		}
		ScrapeType::Main => {
			scrape_main(html)?
		}
		ScrapeType::Changelog => {
			scrape_changelog(html)?
		}
	};

	Ok(EmbedData::new(&title, post_url, &img_url, &preview_text, scrape_type))
}

fn scrape_forum(html: &Html) -> Result<(String, String, String), Box<dyn Error>> {
	const FAIL: NewsError = NewsError::MetaCannotBeScraped(ScrapeType::Forum);
	Ok((
		html.select(&Selector::parse("head>meta:nth-child(5)").map_err(|_| FAIL)?).next().ok_or(FAIL)?.value().attr("content").ok_or(FAIL)?.to_string(),
		"".to_string(),
		html.select(&Selector::parse("head>meta:nth-child(8)").map_err(|_| FAIL)?).next().ok_or(FAIL)?.value().attr("content").ok_or(FAIL)?.to_string()
	))
}

fn scrape_main(html: &Html) -> Result<(String, String, String), Box<dyn Error>> {
	const FAIL: NewsError = NewsError::MetaCannotBeScraped(ScrapeType::Main);
	Ok((
		html.select(&Selector::parse("head>meta:nth-child(13)").map_err(|_| FAIL)?).next().ok_or(FAIL)?.value().attr("content").ok_or(FAIL)?.to_string(),
		scrape_news_image(html),
		sanitize_html(&get_next_selector(html, "p", ScrapeType::Main)?)
	))
}

fn scrape_changelog(html: &Html) -> Result<(String, String, String), Box<dyn Error>> {
	const FAIL: NewsError = NewsError::MetaCannotBeScraped(ScrapeType::Changelog);
	Ok((
		html.select(&Selector::parse("head>meta:nth-child(13)").map_err(|_| FAIL)?).next().ok_or(FAIL)?.value().attr("content").ok_or(FAIL)?.to_string(),
		scrape_news_image(html),
		"The current provided changelog reflects the major changes within the game as part of this Update. Some updates, additions and fixes may not be listed in the provided notes. War Thunder is constantly improving and specific fixes may be implemented without the client being updated.".to_string()
	))
}

/// Returns sufficiently long string as description for embed
fn get_next_selector(html: &Html, selector: &str, scape_type: ScrapeType) -> Result<String, Box<dyn Error>> {
	let selector = Selector::parse(selector).unwrap();
	let selected = html.select(&selector);
	for item in selected {
		if item.inner_html().len() >= 5 {
			return Ok(item.inner_html());
		}
	}
	Err(Box::new(NewsError::MetaCannotBeScraped(scape_type)))
}

// Builds discord ready embed URL from html anchors
fn sanitize_html(html: &str) -> String {
	static SPECIAL_DELIM: char = '🦆'; // Quack quack :D

	let urls = {
		// Splits all parts of the HTML into its a elements
		let left: Vec<&str> = html.split(r#"href=""#).collect();

		let mut finished_urls: Vec<(String, String)> = Vec::new();

		// iterate all items in the split, and extract their corresponding items
		for item in left {
			// abort current iter if it doesnt contain link
			if !item.contains("https") {
				continue;
			}

			// url without the link, such as `https://flareflo.dev` and `>text content </a>`
			let url_split: Vec<&str> = item.split(r#"">"#).collect();

			// Only should have link and alias
			if url_split.len() != 2 {
				continue;
			}
			let url = url_split[0];

			// breaks apart the reminder such as inputs like `>bla bla bla</a>` so that only bla bla remains
			let text = url_split[1].split("</a>").collect::<Vec<&str>>()[0];
			finished_urls.push((text.to_owned(), url.to_owned()));
		}
		finished_urls
	};

	let mut in_escape = false;
	let mut constructed = "".to_owned();

	for (i, char) in html.chars().enumerate() {
		match char {
			'>' => {
				in_escape = false;
			}
			'<' => {
				if html.chars().collect::<Vec<char>>().get(i + 1) == Some(&'a') {
					constructed.push(SPECIAL_DELIM);
					constructed.push(' ');
				}
				in_escape = true;
			}
			_ => {
				if !in_escape {
					constructed.push(char);
				}
			}
		};
	}

	for url in urls {
		constructed = constructed.replace(&url.0, "");
		constructed = constructed.replacen(SPECIAL_DELIM, &format!("[{}]({})", url.0, url.1), 1);
	}

	constructed
}

/// Collects meta image to display for embed
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
	use crate::scrapers::scrape_meta::{sanitize_html, scrape_meta};
	use crate::scrapers::scraper_resources::resources::{request_html, ScrapeType};

	#[tokio::test]
	async fn test_embed_data_main() {
		// let url = "https://warthunder.com/en/news/7598-development-lav-ad-revolving-firepower-en";
		let url = "https://warthunder.com/en/news/7640-event-the-battle-for-arachis-en";
		let html = request_html(url).await.unwrap();

		eprintln!("{:#?}", scrape_meta(&html, ScrapeType::Main, &url.to_owned()));
	}

	#[tokio::test]
	async fn test_embed_data_changelog() {
		let url = "https://warthunder.com/en/game/changelog/current/1352";
		let html = request_html(url).await.unwrap();

		eprintln!("{:#?}", scrape_meta(&html, ScrapeType::Changelog, &url.to_owned()));
	}

	#[test]
	fn test_html_sanitization() {
		static RAW: &str = r#"Together with <a href="https://warthunder.com/en/news/7583-development-dagor-engine-6-5-zoom-in-enhance-it-en">texture upscaling</a> and <a href="https://warthunder.com/en/news/7585-development-dagor-engine-6-5-new-surface-rendering-en">new surface rendering options</a>, the new version of the War Thunder graphic engine brings numerous minor features and improvements. Meet new visuals coming soon in the “Wind of Change” update!"#;
		static ESCAPED: &str = r#"Together with [texture upscaling](https://warthunder.com/en/news/7583-development-dagor-engine-6-5-zoom-in-enhance-it-en)  and [new surface rendering options](https://warthunder.com/en/news/7585-development-dagor-engine-6-5-new-surface-rendering-en) , the new version of the War Thunder graphic engine brings numerous minor features and improvements. Meet new visuals coming soon in the “Wind of Change” update!"#;

		assert_eq!(sanitize_html(RAW), ESCAPED);
	}
}