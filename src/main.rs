use std::thread::sleep;
use std::time;

use log::*;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use rand;
use rand::Rng;

use crate::recent_name_to_index::convert;
use crate::scrapers::forum_news::html_processor_wt_forums;
use crate::scrapers::wt_news::html_processor_wt_news;
use crate::webhook_handler::handle_webhook;

mod webhook_handler;
mod recent_name_to_index;
mod scrapers;

#[tokio::main]
async fn main() {
	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{l} {d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n")))
		.build("log/output.log").unwrap();

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.build(Root::builder()
			.appender("logfile")
			.build(LevelFilter::Info)).unwrap();

	log4rs::init_config(config).unwrap();

	info!("Started client");

	loop {
		let index = convert("warthunder_news");
		let wt_news_content = html_processor_wt_news(index).await;
		if wt_news_content != "fetch_failed" {
			handle_webhook(wt_news_content, index).await;
		};

		let index = convert("forums");
		let forum_news = html_processor_wt_forums(index).await;
		if forum_news != "fetch_failed" {
			handle_webhook(forum_news, index).await;
		};

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		info!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}