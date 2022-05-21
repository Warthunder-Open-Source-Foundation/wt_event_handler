use std::fmt::{Display, Formatter};
use std::fs;

use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::utils::Color;

use crate::{print_log, TOKEN_PATH, WebhookAuth};

#[derive(Debug, Clone, Copy)]
pub struct Statistics {
	pub fetch_counter: usize,
	pub post_counter: usize,
	pub new_news: usize,
	pub errors: usize,
	pub timeouts: usize,

}

impl Display for Statistics {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Fetch count: {}\n\
					Posted news: {}\n\
					New news: {}\n\
					Issues handled: {}\n\
					Sources timed out: {}
					",
			   self.fetch_counter,
			   self.post_counter,
			   self.new_news,
			   self.errors,
			   self.timeouts,
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Incr {
	FetchCounter,
	PostCounter,
	NewNews,
	Errors,
	Timeouts,
}

impl Statistics {
	pub const fn new() -> Self {
		Self {
			fetch_counter: 0,
			post_counter: 0,
			new_news: 0,
			errors: 0,
			timeouts: 0
		}
	}
	pub fn increment(&mut self, incr: Incr) {
		match incr {
			Incr::FetchCounter => { self.fetch_counter += 1 }
			Incr::PostCounter => { self.post_counter += 1 }
			Incr::NewNews => { self.new_news += 1 }
			Incr::Errors => { self.errors += 1 }
			Incr::Timeouts => { self.timeouts += 1 }
		}
	}
	pub async fn post(&mut self) {
		let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		let my_http_client = Http::new(&webhook_auth.statistics_hook.token);

		let webhook = match my_http_client.get_webhook_with_token(webhook_auth.statistics_hook.uid, &webhook_auth.statistics_hook.token).await {
			Err(why) => {
				print_log(&format!("{why}"), 0);
				std::panic::panic_any(why)
			}
			Ok(hook) => hook,
		};

		let embed = Embed::fake(|e| {
			e.title("Statistics for the past time")
			 .color(Color::from_rgb(116, 16, 210))
			 .field("Numbers", format!("{self}"), false)
			 .thumbnail("https://avatars.githubusercontent.com/u/97326911?s=40&v=4")
			 .footer(|f| {
				 f.icon_url("https://warthunder.com/i/favicons/mstile-70x70.png").text("Report bugs/issues: FlareFlo🦆#2800")
			 })
		});

		webhook.execute(my_http_client, false, |w| {
			w.embeds(vec![embed]);
			w
		}).await.unwrap();

		print_log("All statistics are posted", 1);
	}
}