use std::thread::sleep;
use std::time;

use rand;
use rand::Rng;

use crate::coub::html_processor_coub;
use crate::forum_news::html_processor_wt_forums;
use crate::webhook_handler::handle_webhook;
use crate::wt_news::html_processor_wt_news;

mod wt_news;
mod coub;
mod forum_news;
mod webhook_handler;

#[tokio::main]
async fn main() {
	loop {
		let wt_news_content = html_processor_wt_news(0).await;
		if wt_news_content != "fetch_failed" {
			handle_webhook(wt_news_content, 0).await;
		}

		// let coub = html_processor_coub(1).await;
		// handle_webhook(coub, 1).await;

		let forum_news = html_processor_wt_forums(2).await;
		if forum_news != "fetch_failed" {
			handle_webhook(forum_news, 2).await;
		}

		// Cool down to prevent rate limiting and excessive performance impact
		let wait = rand::thread_rng().gen_range(50..70);
		println!("Waiting for {} seconds", wait);
		sleep(time::Duration::from_secs(wait))
	}
}