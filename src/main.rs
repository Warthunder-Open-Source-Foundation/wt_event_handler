use std::{fs, time};
use std::path::Path;
use std::thread::sleep;

use chrono::{Datelike, Timelike};
use chrono::offset::Local;
use log::*;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use rand::Rng;

use crate::json_to_structs::recent::*;
use crate::scrapers::forum_news::html_processor_wt_forums;
use crate::scrapers::wt_changelog::html_processor_wt_changelog;
use crate::scrapers::wt_news::html_processor_wt_news;

mod webhook_handler;
mod scrapers;
mod json_to_structs;


#[tokio::main]
async fn main() {
	if Path::new("log/latest.log").exists() {
		let now = Local::now().format("%Y_%m_%d_%H-%M-%S").to_string();
		fs::rename("log/latest.log", format!("log/old/{}.log", now)).expect("Could not rename latest log file");
	}

	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{l} {d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
		.build("log/latest.log").unwrap();

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.build(Root::builder()
			.appender("logfile")
			.build(LevelFilter::Info)).unwrap();

	log4rs::init_config(config).unwrap();

	println!("Started client");
	info!("Started client");

	let mut recent_data = Recent::read_latest();

	loop {
		let wt_news_content = html_processor_wt_news().await;
		if wt_news_content != "fetch_failed" {
			if recent_data.warthunder_news.is_outdated(&wt_news_content) {
				recent_data.warthunder_news.handle_wt_news_webhook(&wt_news_content).await;
				recent_data.append_latest_warthunder_news(&wt_news_content);
				println!("All wt news hooks are served");
				info!("All wt news hooks are served");
			}
		};

		let wt_changelog = html_processor_wt_changelog().await;
		if wt_changelog != "fetch_failed" {
			if recent_data.warthunder_changelog.is_outdated(&wt_changelog) {
				recent_data.warthunder_changelog.handle_simple_webhook(&wt_changelog).await;
				recent_data.append_latest_warthunder_changelog(&wt_changelog);
				println!("All wt changelog hooks are served");
				info!("All wt changelog hooks are served");
			}
		};

		let forum_news = html_processor_wt_forums().await;
		if forum_news != "fetch_failed" {
			if recent_data.forums.is_outdated(&forum_news) {
				recent_data.forums.handle_simple_webhook(&forum_news).await;
				recent_data.append_latest_warthunder_forums(&forum_news);
				println!("All forum hooks are served");
				info!("All forum hooks are served");
			}
		};

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		info!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}