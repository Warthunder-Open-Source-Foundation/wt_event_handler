use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serenity::http::Http;
use serenity::model::prelude::Embed;
use serenity::utils::Color;
use crate::{PANIC_INFO, print_log};

#[derive(Debug, Clone)]
pub enum NewsError {
	NoUrlOnPost(String),
}

impl Display for NewsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			NewsError::NoUrlOnPost(err_message) => {
				write!(f, "No URL was scraped on target \'{err_message}\'")
			}
		}
	}
}

impl Error for NewsError {}

pub async fn error_webhook(error: Box<dyn Error>) {
	let my_http_client = Http::new_with_token(&PANIC_INFO.token);

	let webhook = match my_http_client.get_webhook_with_token(PANIC_INFO.uid, &PANIC_INFO.token).await {
		Err(why) => {
			print_log(&format!("{why}"), 0);
			std::panic::panic_any(why)
		}
		Ok(hook) => hook,
	};

	let embed = Embed::fake(|e| {
		e.title("News bot error")
			.field("Error information", error, false)
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
	print_log(&format!("Posted webhook for {}", PANIC_INFO.name), 1);
}