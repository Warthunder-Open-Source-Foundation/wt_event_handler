use std::convert::TryFrom;
use std::fs;

use chrono::Local;
use scraper::Selector;

use crate::RECENT_PATH;
use crate::scrapers::scraper_resources::resources::RecentHtmlTarget;
use crate::webhook_handler::print_log;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Recent {
	pub meta: Meta,
	pub warthunder_news: Channel,
	pub warthunder_changelog: Channel,
	pub forums_updates_information: Channel,
	pub forums_project_news: Channel,
	pub forums_notice_board: Channel,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Meta {
	pub timestamp: u64,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Channel {
	pub domain: String,
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
}

impl Recent {
	pub fn append_latest_warthunder_news(&mut self, value: &str) {
		self.warthunder_news.recent_url.push(value.to_owned());
		self.update_timestamp();
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_changelog(&mut self, value: &str) {
		self.warthunder_changelog.recent_url.push(value.to_owned());
		self.update_timestamp();
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_forums_updates_information(&mut self, value: &str) {
		self.forums_updates_information.recent_url.push(value.to_owned());
		self.update_timestamp();
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_forums_project_news(&mut self, value: &str) {
		self.forums_project_news.recent_url.push(value.to_owned());
		self.update_timestamp();
		self.write_latest(value);
	}
	pub fn append_latest_forums_notice_board(&mut self, value: &str) {
		self.forums_notice_board.recent_url.push(value.to_owned());
		self.update_timestamp();
		self.write_latest(value);
	}
	fn update_timestamp(&mut self) {
		self.meta.timestamp = u64::try_from(Local::now().timestamp()).unwrap();
	}
	pub fn read_latest() -> Self {
		let cache_raw_recent = fs::read_to_string(RECENT_PATH).expect("Cannot read file");
		let recent: Self = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");
		recent
	}
	fn write_latest(&self, value: &str) {
		let write = serde_json::to_string_pretty(self).unwrap();
		fs::write(RECENT_PATH, write).expect("Couldn't write to recent file");
		print_log(&format!("Written {} to file", value), 1);
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
