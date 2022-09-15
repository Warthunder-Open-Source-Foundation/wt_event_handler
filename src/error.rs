use std::fmt::{Debug};
use thiserror::Error as ThisError;

use serenity::http::Http;
use serenity::model;
use serenity::model::prelude::Embed;
use serenity::utils::Color;
use tracing::{error, warn};

use crate::{PANIC_INFO};
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[derive(Debug, ThisError)]
#[allow(dead_code)]
pub enum NewsError {
	// LHS: Url, RHS: Post Url
	#[error("NoUrlOnPost: {0} returned a document, but no URL was found.\nDocument: {1}")]
	NoUrlOnPost(String, String),

	/// LHS: ScrapeType, RHS: Post URL
	#[error("MetaCannotBeScraped: The meta data for \'{0}\' cannot be collected, falling back to defaults")]
	MetaCannotBeScraped(ScrapeType),

	/// ScrapeType, Error message, Timestamp until retiming
	#[error("SourceTimeout: Source \'{0}\' timed out and will not be fetched until <t:{1}>. \nError message: \"{2}\"")]
	SourceTimeout(ScrapeType, String, i64),

	/// Selector in question
	#[error("BadSelector: The selector \'{0}\' failed to parse")]
	BadSelector(String),

	/// Month in string form pre-parse
	#[error("MonthParse: \'{0}\' failed to parse into month")]
	MonthParse(String),

	/// LHS: Selector, RHS: Url
	#[error("SelectedNothing: Selector: \'{0}\' found no item.\nDocument: {1}")]
	SelectedNothing(String, String),

	#[error(transparent)]
	SerenityError(#[from] serenity::Error),

	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}

// Extra text is not an option thanks to type-system fuckery not permitting the type Option contain a impl statement
pub async fn error_webhook<T>(error: &NewsError, extra_text: &T, can_recover: bool)
where T: ToString + ?Sized
{
	ship_error_webhook(error.to_string(), extra_text,can_recover).await;
	warn!("Posted panic webhook for {}", PANIC_INFO.name);
}

// Extra text is not an option thanks to type-system fuckery not permitting the type Option contain a impl statement
pub async fn ship_error_webhook<T>(input: String, extra_text: &T, can_recover: bool)
	where T: ToString + ?Sized
{
	let my_http_client = Http::new(&PANIC_INFO.token);

	let webhook = match my_http_client.get_webhook_with_token(PANIC_INFO.uid, &PANIC_INFO.token).await {
		Err(why) => {
			error!("{why}");
			std::panic::panic_any(why)
		}
		Ok(hook) => hook,
	};

	let embed = Embed::fake(|e| {
		let e= e.title(if can_recover {
			"A recoverable error occurred"
		} else {
			"A non-recoverable error occurred"
		}
		)
		 .field("Core error information", input, false)
		 .color(Color::from_rgb(116, 16, 210))
			.timestamp(model::Timestamp::now())
		 .footer(|f| {
			 f.icon_url("https://warthunder.com/i/favicons/mstile-70x70.png").text("Report bugs/issues: FlareFlo🦆#2800")
		 });
		if  extra_text.to_string() != "" {
			e.field("Hint / details", extra_text.to_string(), false);
		};
		e
	});

	webhook.execute(my_http_client, false, |w| {
		w.embeds(vec![embed]);
		w
	}).await.unwrap();
}