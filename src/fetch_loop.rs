use std::error::Error;
use std::fs;
use std::process::exit;
use std::time::Duration;
use lazy_static::lazy_static;

use tokio::sync::Mutex;

use crate::error::{error_webhook, NewsError};
use crate::json::recent::Recent;
use crate::scrapers::html_processing::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::statistics::{Incr, increment, Statistics};
use crate::timeout::Timeout;
use crate::logging::{LogLevel, print_log};

const FETCH_DELAY: u64 = 48;

pub const STAT_COOLDOWN_HOURS: u64 = 24;
// in seconds
const STAT_COOL_DOWN: u64 = 60 * 60 * STAT_COOLDOWN_HOURS;


lazy_static!{
	pub static ref STATS: Mutex<Statistics> = Mutex::new(Statistics::new());
}

pub async fn fetch_loop(hooks: bool, write_files: bool) {
	// First run of the program will fetch everything with no delay
	let mut oneshot = true;
	let mut recent_data = Recent::read_latest();

	let mut timeouts = Timeout::new();

	// Spawn statistics thread
	tokio::task::spawn(async {
		loop {
			tokio::time::sleep(Duration::from_secs(STAT_COOL_DOWN)).await;
			let mut lock = STATS.lock().await;
			lock.post().await;
			lock.reset();
			// Not sure if a loops end counts as termination here, dropping juuuuuuuuuust to make sure
			drop(lock);
		}
	});

	loop {
		for source in &mut recent_data.sources {
			if !timeouts.is_timed_out(&source.name) {
				increment(Incr::FetchCounter).await;
				match html_processor(source).await {
					Ok(content) => {
						if source.is_new(&content.url) {
							if hooks {
								source.handle_webhooks(&content, true, source.scrape_type).await;
							}
							if write_files {
								source.store_recent(&content.url);
							}
							increment(Incr::NewNews).await;
						}
					}
					Err(e) => {
						increment(Incr::Errors).await;
						handle_err(e, source.scrape_type, source.name.clone(), &mut timeouts, hooks).await;
					}
				}
			}
			if oneshot {
				print_log("Skipping sleep for oneshot", LogLevel::Info);
			} else {
				print_log(&format!("Waiting for {FETCH_DELAY} seconds"), LogLevel::Info);
				tokio::time::sleep(Duration::from_secs(FETCH_DELAY)).await;
			}
		}
		oneshot = false;
		recent_data.save();
		//Aborts program after running without hooks
		if !hooks || !write_files {
			exit(0);
		}
	}
}

async fn handle_err(e: Box<dyn Error>, scrape_type: ScrapeType, source: String, timeouts: &mut Timeout, hooks: bool) {
	let crash_and_burn = |e: Box<dyn Error>| async move {
		if hooks {
			error_webhook(&e, false).await;
		}
		panic!("{}", e);
	};

	let time_out = |send, msg: String| async move {
		let now = chrono::offset::Utc::now().timestamp();
		let then = now + (60 * 30);
		if send {
			error_webhook(&Box::new(NewsError::SourceTimeout(scrape_type, msg, then)).into(), true).await;
		}
		let _ = &timeouts.time_out(source, then);
	};

	print_log(&e.to_string(), LogLevel::Error);
	match () {
		_ if let Some(e) = e.downcast_ref::<reqwest::Error>() => {
			let e: &reqwest::Error = e;

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
					// Timeouts happen too often, they are no longer printed out status channels
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
		_ if let Some(variant) = e.downcast_ref::<NewsError>() => {
			match variant {
				NewsError::NoUrlOnPost(name, html) => {
					let now = chrono::Local::now().timestamp();
					let sanitized_url = name.replace('/', "_").replace(':', "_");
					drop(fs::write(&format!("/log/err_html/{sanitized_url}_{now}.html"), html));
					time_out(true, "no_url_on_post".to_owned()).await;
				}
				_ => {
					crash_and_burn(e).await;
				}
			}
		}
		_ => {
			crash_and_burn(e).await;
		}
	}
}