use log::{error, info};
use reqwest::get;
use scraper::Html;

pub async fn request_html(url: &str) -> Option<Html> {
	println!("Fetching data from {}", &url);
	info!("Fetching data from {}", &url);

	let html;
	if let Ok(raw_html) = get(url).await {
		if let Ok(text) = raw_html.text().await {
			html = Html::parse_document(text.as_str());
			return Some(html);
		}
		return None;
	}
	return None;
}

pub fn fetch_failed() -> Option<String> {
	println!("Fetch failed");
	error!("Fetch failed");
	None
}