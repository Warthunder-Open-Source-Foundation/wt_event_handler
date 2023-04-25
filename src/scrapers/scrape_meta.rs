use scraper::Html;

use crate::embed::{EmbedData, EMPTY_IMG};
use crate::error::NewsError;
use crate::scrapers::scraper_resources::html_util::{ElemUtil, format_selector, HtmlUtil};
use crate::scrapers::scraper_resources::resources::ScrapeType;

/// Collects embed information from page
pub fn scrape_meta(html: &Html, scrape_type: ScrapeType, post_url: &str) -> Result<EmbedData, NewsError> {
	let (title, img_url, preview_text) = match scrape_type {
		ScrapeType::Forum => {
			let title_elem = html.select_first("head>meta:nth-child(5)", post_url)?;
			let preview_elem = html.select_first("head>meta:nth-child(8)", post_url)?;
			(
				title_elem.select_attribute("content", post_url)?,
				String::new(),
				preview_elem.select_attribute("content", post_url)?
			)
		}
		ScrapeType::Main => {
			let title_elem = html.select_first("head>meta:nth-child(13)", post_url)?;
			(
				title_elem.select_attribute("content", post_url)?,
				scrape_news_image(html).unwrap_or(EMPTY_IMG.to_owned()),
				sanitize_html(&get_next_selector(html, "p", ScrapeType::Main, post_url)?)
			)
		}
		ScrapeType::Changelog => {
			let title_elem = html.select_first("head>meta:nth-child(13)", post_url)?;
			(
				title_elem.select_attribute("content", post_url)?,
				scrape_news_image(html).unwrap_or(EMPTY_IMG.to_owned()),
				"The current provided changelog reflects the major changes within the game as part of this Update. Some updates, additions and fixes may not be listed in the provided notes. War Thunder is constantly improving and specific fixes may be implemented without the client being updated.".to_owned()
			)
		}
	};

	Ok(EmbedData::new(&title, post_url, &img_url, &preview_text, scrape_type))
}

/// Returns sufficiently long string as description for embed
fn get_next_selector(html: &Html, selector: &str, scrape_type: ScrapeType, post_url: &str) -> Result<String, NewsError> {
	let selector = format_selector(selector)?;
	let selected = html.select(&selector);
	for item in selected {
		if item.inner_html().len() >= 10 {
			return Ok(item.inner_html());
		}
	}
	Err(NewsError::MetaCannotBeScraped(scrape_type, post_url.to_owned()))
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
	let mut constructed = String::new();

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
fn scrape_news_image(html: &Html) -> Result<String, NewsError> {
	let mut actual = String::new();
	for item in html.select(&format_selector("meta, img")?) {
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
	Ok(actual)
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

	#[tokio::test]
	async fn test_embed_data_fixed_url() {
		let url = "https://warthunder.com/en/news/8199-it-s-fixed-73-en";
		let html = request_html(url).await.unwrap();

		eprintln!("{:#?}", scrape_meta(&html, ScrapeType::Main, &url.to_owned()));
	}

	#[test]
	fn test_html_sanitization() {
		static RAW: &str = r#"Together with <a href="https://warthunder.com/en/news/7583-development-dagor-engine-6-5-zoom-in-enhance-it-en">texture upscaling</a> and <a href="https://warthunder.com/en/news/7585-development-dagor-engine-6-5-new-surface-rendering-en">new surface rendering options</a>, the new version of the War Thunder graphic engine brings numerous minor features and improvements. Meet new visuals coming soon in the “Wind of Change” update!"#;
		static ESCAPED: &str = r#"Together with [texture upscaling](https://warthunder.com/en/news/7583-development-dagor-engine-6-5-zoom-in-enhance-it-en)  and [new surface rendering options](https://warthunder.com/en/news/7585-development-dagor-engine-6-5-new-surface-rendering-en) , the new version of the War Thunder graphic engine brings numerous minor features and improvements. Meet new visuals coming soon in the “Wind of Change” update!"#;

		assert_eq!(sanitize_html(RAW), ESCAPED);
	}
}