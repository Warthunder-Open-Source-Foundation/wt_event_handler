use std::fs;
use std::process::exit;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::error::{error_webhook, NewsError};
use crate::json::sources::Sources;
use crate::scrapers::html_processing::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::statistics::{Incr, increment, Statistics};
use crate::timeout::Timeout;

const FETCH_DELAY: u64 = 40;

pub const STAT_COOLDOWN_HOURS: u64 = 24;
// in seconds
const STAT_COOL_DOWN: u64 = 60 * 60 * STAT_COOLDOWN_HOURS;


lazy_static! {
	pub static ref STATS: Mutex<Statistics> = Mutex::new(Statistics::new());
}

pub async fn fetch_loop(hooks: bool) {
	let mut sources = Sources::build().await.expect("I fucked up my soup");

	#[cfg(debug_assertions)]
	sources.debug_remove_tracked_urls::<&[&str]>(&[]);

	let mut timeouts = Timeout::new();

	// Spawn statistics thread
	tokio::task::spawn(async {
		warn!("Spawned logging thread");
		loop {
			tokio::time::sleep(Duration::from_secs(STAT_COOL_DOWN)).await;
			let mut stats = STATS.lock().await;
			stats.post().await;
			stats.reset();
		}
	});

	// Responsible for shutting down tokio-parent / sibling processes
	tokio::spawn(async move {
		tokio::signal::ctrl_c().await.unwrap();
		exit(-1);
	});

	// let cloned_database = database.clone();
	// tokio::spawn( async move {
	// 	let start = Instant::now();
	// 	for _ in 0..100000 {
	// 		cloned_database.get_latest_timestamp();
	// 	}
	// 	println!("Requests per second: {:.2}", 100000.0 / start.elapsed().as_millis() as f64 * 1000.0);
	// 	exit(0);
	// });

	loop {
		for source in &mut sources.sources {
			if !timeouts.is_timed_out(&source.name) {
				increment(Incr::FetchCounter).await;
				match html_processor(source).await {
					Ok(news) => {
						for news_embed in &news {
							if hooks {
								news_embed.handle_webhooks(true, source.scrape_type).await;
							}
							increment(Incr::NewNews).await;
						}

						source.store_recent(news.iter().map(|new| &new.url));
					}
					Err(e) => {
						increment(Incr::Errors).await;
						handle_err(e, source.scrape_type, source.name.clone(), &mut timeouts, hooks).await;
					}
				}
			}
			info!("Waiting for {FETCH_DELAY} seconds");
			tokio::time::sleep(Duration::from_secs(FETCH_DELAY)).await;
		}


		//Aborts program after running without hooks
		if !hooks {
			exit(0);
		}
	}
}

/// Throws error as webhook, times out pages accordingly and terminates program if unrecoverable
async fn handle_err(e: NewsError, scrape_type: ScrapeType, source: String, timeouts: &mut Timeout, hooks: bool) {
	error!("{e}");
	let crash_and_burn = |e: NewsError| async move {
		if hooks {
			error_webhook(&e, "The bot is now offline and needs investigation", false).await;
		}
		panic!("{:?}", e);
	};

	let time_out = |send_webhook_error_message, msg: String| async move {
		let now = chrono::offset::Utc::now().timestamp();
		let then = now + (60 * 30);
		if send_webhook_error_message {
			error_webhook(&NewsError::SourceTimeout(scrape_type, msg, then), "", true).await;
		}
		let _ = &timeouts.time_out(source, then);
	};

	#[allow(clippy::match_wildcard_for_single_variants)]
	match e {
		NewsError::NoUrlOnPost(name, html) => {
			let now = chrono::Local::now().timestamp();
			let sanitized_url = name.replace(['/', ':'], "_");
			drop(fs::write(&format!("/log/err_html/{sanitized_url}_{now}.html"), html));
			time_out(true, "no_url_on_post".to_owned()).await;
		}
		NewsError::MetaCannotBeScraped(_, ref url) => {
			error_webhook(&e, &format!("The [URL]({url}) did not return meta-data"), true).await;
		}
		NewsError::SourceTimeout(_, _, _) => {
			// Dont do anything as it should've been handled earlier
		}
		NewsError::BadSelector(ref selector) => {
			error_webhook(&e, &format!("Selector: {selector}"), true).await;
		}
		NewsError::MonthParse(_) => {
			time_out(true, e.to_string()).await;
		}
		NewsError::SelectedNothing(source, _) => {
			time_out(true, source).await;
		}
		NewsError::SerenityError(_) => {
			error_webhook(&e, "", true).await;
		}
		NewsError::Reqwest(e) => {
			let status = e.status();
			let status_text = if let Some(status) = status {
				format!("status: {status} was returned and initiated:")
			} else {
				"no status code related error was returned and initiated:".to_owned()
			};
			match () {
				_ if e.is_builder() => {
					time_out(true, format!("{status_text} reqwest_bad_builder: {e}")).await;
				}
				_ if e.is_redirect() => {
					time_out(true, format!("{status_text} reqwest_bad_redirect: {e}")).await;
				}
				_ if e.is_status() => {
					time_out(true, format!("{status_text} reqwest_bad_status_{e}: {e}")).await;
				}
				_ if e.is_timeout() => {
					// Timeouts happen too often, they are no longer printed out to status channels
					time_out(false, format!("{status_text} reqwest_timeout: {e}")).await;
				}
				_ if e.is_request() => {
					time_out(true, format!("{status_text} reqwest_bad_request: {e}")).await;
				}
				_ if e.is_connect() => {
					time_out(true, format!("{status_text} reqwest_bad_connect: {e}")).await;
				}
				_ if e.is_body() => {
					time_out(true, format!("{status_text} reqwest_bad_body: {e}")).await;
				}
				_ if e.is_decode() => {
					time_out(true, format!("{status_text} reqwest_bad_body: {e}")).await;
				}
				_ => {
					time_out(true, format!("{status_text} reqwest_everything_bad: {e}")).await;
				}
			}
		}
		NewsError::SerdeJson(_) => {
			todo!("Impelent this!");
		}
		_ => {
			crash_and_burn(e).await;
		}
	}
}
