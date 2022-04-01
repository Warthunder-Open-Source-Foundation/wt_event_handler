use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use log::info;

use crate::json::recent::Recent;
use crate::scrapers::html_processing::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;
use crate::webhook_handler::print_log;

pub async fn fetch_loop(hooks: bool, write_files: bool) {
	let mut recent_data = Recent::read_latest();

	loop {
		if let Some(wt_news_content) = html_processor(&recent_data.warthunder_news, ScrapeType::Main).await {
			if recent_data.warthunder_news.is_outdated(&wt_news_content.url) {
				if hooks {
					recent_data.warthunder_news.handle_webhook(wt_news_content.clone(), true, ScrapeType::Main).await;
				}
				if write_files {
					recent_data.append_latest_warthunder_news(&wt_news_content.url);
				}
				print_log("All wt news hooks are served", 2);
				if hooks && write_files {
					continue;
				}
			}
		};

		if let Some(wt_changelog) = html_processor(&recent_data.warthunder_changelog, ScrapeType::Changelog).await {
			if recent_data.warthunder_changelog.is_outdated(&wt_changelog.url) {
				if hooks {
					recent_data.warthunder_changelog.handle_webhook(wt_changelog.clone(), true, ScrapeType::Changelog).await;
				}
				if write_files {
					recent_data.append_latest_warthunder_changelog(&wt_changelog.url);
				}
				print_log("All wt changelog hooks are served", 1);
				if hooks && write_files {
					continue;
				}
			}
		};

		if let Some(forum_news_updates_information) = html_processor(&recent_data.forums_updates_information, ScrapeType::Forum).await {
			if recent_data.forums_updates_information.is_outdated(&forum_news_updates_information.url) {
				if hooks {
					recent_data.forums_updates_information.handle_webhook(forum_news_updates_information.clone(), true, ScrapeType::Forum).await;
				}
				if write_files {
					recent_data.append_latest_warthunder_forums_updates_information(&forum_news_updates_information.url);
				}
				print_log("All forum_updates_information hooks are served", 2);
				if hooks && write_files {
					continue;
				}
			}
		};

		if let Some(forum_news_project_news) = html_processor(&recent_data.forums_project_news, ScrapeType::Forum).await {
			if recent_data.forums_project_news.is_outdated(&forum_news_project_news.url) {
				if hooks {
					recent_data.forums_project_news.handle_webhook(forum_news_project_news.clone(), true, ScrapeType::Forum).await;
				}
				if write_files {
					recent_data.append_latest_warthunder_forums_project_news(&forum_news_project_news.url);
				}
				print_log("All forum_project_news hooks are served", 2);
				if hooks && write_files {
					continue;
				}
			}
		}

		if let Some(forums_notice_board) = html_processor(&recent_data.forums_notice_board, ScrapeType::Forum).await {
			if recent_data.forums_notice_board.is_outdated(&forums_notice_board.url) {
				if hooks {
					recent_data.forums_notice_board.handle_webhook(forums_notice_board.clone(), true, ScrapeType::Forum).await;
				}
				if write_files {
					recent_data.append_latest_forums_notice_board(&forums_notice_board.url);
				}
				print_log("All forums_notice_board hooks are served", 2);

				if hooks && write_files {
					continue;
				}
			}
		}

		//Aborts program after running without hooks
		if !hooks || !write_files {
			exit(0);
		}

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = Duration::from_secs(60);
		println!("{} Waiting for 60 seconds", chrono::Local::now());
		info!("{} Waiting for 60 seconds", chrono::Local::now());
		sleep(wait);
	}
}