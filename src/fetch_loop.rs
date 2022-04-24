use std::error::Error;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use crate::error::{error_webhook, NewsError};

use crate::json::recent::Recent;
use crate::scrapers::html_processing::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::timeout::Timeout;
use crate::webhook_handler::print_log;

pub async fn fetch_loop(hooks: bool, write_files: bool) {
	let mut recent_data = Recent::read_latest();

	let mut timeouts = Timeout::new();

	loop {
		if !timeouts.is_timed_out("warthunder_news") {
			match html_processor(&recent_data.warthunder_news, ScrapeType::Main).await {
				Ok(wt_news_content) => {
					if recent_data.warthunder_news.is_outdated(&wt_news_content.url) {
						if hooks {
							recent_data.warthunder_news.handle_webhook(wt_news_content.clone(), true, ScrapeType::Main).await;
						}
						if write_files {
							recent_data.append_latest_warthunder_news(&wt_news_content.url);
						}
						print_log("All wt news hooks are served", 2);
					}
				}
				Err(e) => {
					handle_err(e, ScrapeType::Main, "warthunder_news".to_owned(), &mut timeouts, hooks).await;
				}
			};
		}

		if !timeouts.is_timed_out("warthunder_changelog") {
			match html_processor(&recent_data.warthunder_changelog, ScrapeType::Changelog).await {
				Ok(wt_changelog) => {
					if recent_data.warthunder_changelog.is_outdated(&wt_changelog.url) {
						if hooks {
							recent_data.warthunder_changelog.handle_webhook(wt_changelog.clone(), true, ScrapeType::Changelog).await;
						}
						if write_files {
							recent_data.append_latest_warthunder_changelog(&wt_changelog.url);
						}
						print_log("All wt changelog hooks are served", 1);
					}
				}
				Err(e) => {
					handle_err(e, ScrapeType::Changelog, "warthunder_changelog".to_owned(), &mut timeouts, hooks).await;
				}
			};
		}

		if !timeouts.is_timed_out("forums_updates_information") {
			match html_processor(&recent_data.forums_updates_information, ScrapeType::Forum).await {
				Ok(forum_news_updates_information) => {
					if recent_data.forums_updates_information.is_outdated(&forum_news_updates_information.url) {
						if hooks {
							recent_data.forums_updates_information.handle_webhook(forum_news_updates_information.clone(), true, ScrapeType::Forum).await;
						}
						if write_files {
							recent_data.append_latest_warthunder_forums_updates_information(&forum_news_updates_information.url);
						}
						print_log("All forum_updates_information hooks are served", 2);
					}
				}
				Err(e) => {
					handle_err(e, ScrapeType::Forum, "forums_updates_information".to_owned(), &mut timeouts, hooks).await;
				}
			};
		}

		if !timeouts.is_timed_out("forums_project_news") {
			match html_processor(&recent_data.forums_project_news, ScrapeType::Forum).await {
				Ok(forum_news_project_news) => {
					if recent_data.forums_project_news.is_outdated(&forum_news_project_news.url) {
						if hooks {
							recent_data.forums_project_news.handle_webhook(forum_news_project_news.clone(), true, ScrapeType::Forum).await;
						}
						if write_files {
							recent_data.append_latest_warthunder_forums_project_news(&forum_news_project_news.url);
						}
						print_log("All forum_project_news hooks are served", 2);
					}
				}
				Err(e) => {
					handle_err(e, ScrapeType::Forum, "forums_project_news".to_owned(), &mut timeouts, hooks).await;
				}
			};
		}

		if !timeouts.is_timed_out("forums_notice_board") {
			match html_processor(&recent_data.forums_notice_board, ScrapeType::Forum).await {
				Ok(forums_notice_board) => {
					if recent_data.forums_notice_board.is_outdated(&forums_notice_board.url) {
						if hooks {
							recent_data.forums_notice_board.handle_webhook(forums_notice_board.clone(), true, ScrapeType::Forum).await;
						}
						if write_files {
							recent_data.append_latest_forums_notice_board(&forums_notice_board.url);
						}
						print_log("All forums_notice_board hooks are served", 2);
					}
				}
				Err(e) => {
					handle_err(e, ScrapeType::Forum, "forums_notice_board".to_owned(), &mut timeouts, hooks).await;
				}
			};
		}

		//Aborts program after running without hooks
		if !hooks || !write_files {
			exit(0);
		}

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = Duration::from_secs(60);
		print_log("Waiting for 60 seconds", 2);
		sleep(wait);
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

	match () {
		_ if e.downcast_ref::<reqwest::Error>().is_some() => {
			let now = chrono::offset::Utc::now().timestamp();
			let then = now + (60 * 60);
			error_webhook(&Box::new(NewsError::SourceTimeout(scrape_type, then)).into(), true).await;
			let _ = &timeouts.time_out(source, then);
		}
		_ => {
			crash_and_burn(e).await;
		}
	}
}