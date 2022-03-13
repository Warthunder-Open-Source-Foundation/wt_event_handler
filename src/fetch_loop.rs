use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use log::info;

use crate::json::recent::Recent;
use crate::scrapers::main_news::html_processor;
use crate::scrapers::scraper_resources::resources::ScrapeType;

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
				println!("All wt news hooks are served");
				info!("All wt news hooks are served");
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
				println!("All wt changelog hooks are served");
				info!("All wt changelog hooks are served");
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
				println!("All forum_updates_information hooks are served");
				info!("All forum_updates_information hooks are served");
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
				println!("All forum_project_news hooks are served");
				info!("All forum_project_news hooks are served");
				if hooks && write_files {
					continue;
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
}