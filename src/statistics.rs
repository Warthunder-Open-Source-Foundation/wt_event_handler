use std::fmt::{Display, Formatter};

use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::utils::Color;
use tracing::{error, warn};

use crate::fetch_loop::{STAT_COOLDOWN_HOURS, STATS};
use crate::WEBHOOK_AUTH;

#[derive(Debug, Clone, Copy)]
/// Counts statistics during runtime
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
/// Used to define which statistic to increment
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
			timeouts: 0,
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
	pub fn reset(&mut self) {
		self.fetch_counter = 0;
		self.post_counter = 0;
		self.new_news = 0;
		self.errors = 0;
		self.timeouts = 0;
	}
	pub async fn post(&mut self) {
		let my_http_client = Http::new(&WEBHOOK_AUTH.statistics_hook.token);

		let webhook = match my_http_client.get_webhook_with_token(WEBHOOK_AUTH.statistics_hook.uid, &WEBHOOK_AUTH.statistics_hook.token).await {
			Err(why) => {
				error!("{why}");
				std::panic::panic_any(why)
			}
			Ok(hook) => hook,
		};

		let embed = Embed::fake(|e| {
			e.title(format!("Statistics for the past {} hours", STAT_COOLDOWN_HOURS))
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

		warn!("All statistics are posted");
	}
}

pub async fn increment(incr: Incr) {
	let mut lock = STATS.lock().await;
	lock.increment(incr);
}