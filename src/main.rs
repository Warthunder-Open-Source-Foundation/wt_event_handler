use std::thread::sleep;
use std::time;

use rand;
use rand::Rng;

use crate::scrapers::wt_news::html_processor_wt_news;
use crate::scrapers::forum_news::html_processor_wt_forums;
use crate::webhook_handler::handle_webhook;
use crate::recent_name_to_index::convert;

mod webhook_handler;
mod recent_name_to_index;
mod scrapers;

#[tokio::main]
async fn main() {
	loop {
		let index = convert("warthunder_news");
		let wt_news_content = html_processor_wt_news(index).await;
		if wt_news_content != "fetch_failed" {
			handle_webhook(wt_news_content, index).await;
		};

		// let coub = html_processor_coub(1).await;
		// handle_webhook(coub, 1).await;

		let index = convert("forums");
		let forum_news = html_processor_wt_forums(index).await;
		if forum_news != "fetch_failed" {
			handle_webhook(forum_news, index).await;
		};

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}