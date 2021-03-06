use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serenity::http::Http;
use serenity::model::prelude::Embed;
use serenity::utils::Color;

use crate::{PANIC_INFO, print_log};
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Debug, Clone)]
pub enum NewsError {
	// URL which was fetched and the HTML returned
	NoUrlOnPost(String, String),
	MetaCannotBeScraped(ScrapeType),
	SourceTimeout(ScrapeType, String, i64),
}

impl Display for NewsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			NewsError::NoUrlOnPost(url, html) => {
				write!(f, "NoUrlOnPost: {url} returned a document, but no URL was found.\nDocument: {html}")
			}
			NewsError::MetaCannotBeScraped(scrape_type) => {
				write!(f, "MetaCannotBeScraped: The meta data for \'{scrape_type}\' cannot be collected, falling back to defaults")
			}
			NewsError::SourceTimeout(scrape_type, msg, timestamp) => {
				write!(f, "SourceTimeout: Source \'{scrape_type}\' timed out and will not be fetched until <t:{timestamp}>. \nError message: \"{msg}\"")
			}
		}
	}
}

impl Error for NewsError {}

// Clippy error
#[allow(clippy::borrowed_box)]
pub async fn error_webhook(error: &Box<dyn Error>, can_recover: bool) {
	let my_http_client = Http::new(&PANIC_INFO.token);

	let webhook = match my_http_client.get_webhook_with_token(PANIC_INFO.uid, &PANIC_INFO.token).await {
		Err(why) => {
			print_log(&format!("{why}"), 0);
			std::panic::panic_any(why)
		}
		Ok(hook) => hook,
	};

	let embed = Embed::fake(|e| {
		e.title(if can_recover {
			"A recoverable error occurred"
		} else {
			"A non-recoverable error occurred"
		}
		)
		 .field("More information", error, false)
		 .description(format!("Fetched on: <t:{}>", chrono::offset::Local::now().timestamp()))
		 .color(Color::from_rgb(116, 16, 210))
		 .footer(|f| {
			 f.icon_url("https://warthunder.com/i/favicons/mstile-70x70.png").text("Report bugs/issues: FlareFlo🦆#2800")
		 })
	});

	webhook.execute(my_http_client, false, |w| {
		w.embeds(vec![embed]);
		w
	}).await.unwrap();
	print_log(&format!("Posted panic webhook for {}", PANIC_INFO.name), 1);
}