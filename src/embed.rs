use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Clone, Debug)]
pub struct EmbedData {
	pub scrape_type: ScrapeType,
	pub title: String,
	pub url: String,
	pub img_url: String,
	pub preview_text: String,
}

impl EmbedData {
	pub fn new(title: &str, url: &str, img_url: &str, preview_text: &str, scrape_type: ScrapeType) -> Self {
		let sanitized_img_url = img_url.replace(' ', "%20").to_owned();
		Self {
			scrape_type,
			title: title.to_owned(),
			url: url.to_owned(),
			img_url: sanitized_img_url,
			preview_text: preview_text.to_owned()
		}
	}
	pub fn test() -> Self {
		Self {
			scrape_type: ScrapeType::Main,
			title: "This is a test message".to_owned(),
			url: "https://github.com/Warthunder-Open-Source-Foundation/wt_event_handler".to_owned(),
			img_url: "https://avatars.githubusercontent.com/u/97326911?s=200&v=4".to_owned(),
			preview_text: "Test preview text".to_owned()
		}
	}
}