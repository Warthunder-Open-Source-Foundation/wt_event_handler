use std::convert::TryFrom;
use std::fs;

use chrono::Local;
use scraper::Selector;

use crate::RECENT_PATH;
use crate::scrapers::scraper_resources::resources::{RecentHtmlTarget, ScrapeType};
use crate::webhook_handler::print_log;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Recent {
	pub meta: Meta,
	pub sources: Vec<Channel>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Meta {
	pub timestamp: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Channel {
	pub name: String,
	pub domain: String,
	pub scrape_type: ScrapeType,
	pub selector: String,
	pub pin: String,
	pub recent_url: Vec<String>,
}

impl Channel {
	pub fn is_outdated(&self, value: &str) -> bool {
		if self.recent_url.contains(&value.to_owned()) {
			print_log("Content was recently fetched and is not new", 2);
			false
		} else {
			print_log("New post found, hooking now", 1);
			true
		}
	}
	pub fn append_latest(&mut self, value: &str) {
		self.recent_url.push(value.to_owned());
	}
}

impl Recent {
	pub fn save(&mut self) {
		self.update_timestamp();

		let write = serde_json::to_string_pretty(self).unwrap();
		fs::write(RECENT_PATH, write).expect("Couldn't write to recent file");
		print_log("Saved recent to file", 1);
	}
	fn update_timestamp(&mut self) {
		self.meta.timestamp = u64::try_from(Local::now().timestamp()).unwrap();
	}
	pub fn read_latest() -> Self {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let recent: Self = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");
		recent
	}
}

pub fn format_selector(main: &Channel, which: &RecentHtmlTarget, index: u32) -> Selector {
	return match which {
		RecentHtmlTarget::Pin => {
			Selector::parse(&*format!("{}{}{}", &*main.pin.split_whitespace().collect::<Vec<&str>>()[0], index, &*main.pin.split_whitespace().collect::<Vec<&str>>()[1])).unwrap()
		}
		RecentHtmlTarget::Post => {
			Selector::parse(&*format!("{}{}{}", &*main.selector.split_whitespace().collect::<Vec<&str>>()[0], index, &*main.selector.split_whitespace().collect::<Vec<&str>>()[1])).unwrap()
		}
	};
}
