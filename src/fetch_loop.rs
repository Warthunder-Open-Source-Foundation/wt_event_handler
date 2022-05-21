use std::error::Error;
use std::fs;
use std::lazy::SyncLazy;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::error::{error_webhook, NewsError};
use crate::json::recent::Recent;
use crate::scrapers::html_processing::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::statistics::{Incr, Statistics};
use crate::timeout::Timeout;
use crate::webhook_handler::print_log;
use crate::{TOKEN_PATH, WebhookAuth};

const FETCH_DELAY: u64 = 48;

pub static STATS: SyncLazy<Mutex<Statistics>> = SyncLazy::new(||
	Mutex::new(Statistics::new())
);

pub async fn fetch_loop(hooks: bool, write_files: bool) {
	// First run of the program will fetch everything with no delay
	let mut oneshot = true;
	let mut recent_data = Recent::read_latest();

	let mut timeouts = Timeout::new();

	// Spawn statistics thread
	tokio::task::spawn( async {
		let token_raw = fs::read_to_string(TOKEN_PATH).expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		loop {
			tokio::time::sleep(Duration::from_secs(webhook_auth.statistics_hook.time_between_post / 60)).await;
			let mut lock = STATS.lock().await;
			lock.post().await;
			// Not sure if a loops end counts as termination here, dropping juuuuuuuuuust to make sure
			drop(lock);
		}
	});

	loop {
		for source in &mut recent_data.sources {
			if !timeouts.is_timed_out(&source.name) {
				STATS.lock().await.increment(Incr::FetchCounter);
				match html_processor(source).await {
					Ok(content) => {
						if source.is_new(&content.url) {
							if hooks {
								source.handle_webhook(&content, true, source.scrape_type).await;
							}
							if write_files {
								source.append_latest(&content.url);
							}
							STATS.lock().await.increment(Incr::NewNews);
						}
					}
					Err(e) => {
						STATS.lock().await.increment(Incr::Errors);
						handle_err(e, source.scrape_type, source.name.clone(), &mut timeouts, hooks).await;
					}
				}
			}
			if oneshot {
				print_log("Skipping sleep for oneshot", 2);
			} else {
				print_log(&format!("Waiting for {FETCH_DELAY} seconds"), 2);
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
		print_log(&e.to_string(), 0);
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
					time_out(true, format!("{status_text} reqwest_bad_status_{e}: {e}", )).await;
				}
				_ if e.is_timeout() => {
					// Timeouts happen too often, they are no longer returned
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
				NewsError::NoUrlOnPost(_) => {
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