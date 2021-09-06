use std::fs;

use log::{info, warn};
use scraper::Selector;

use crate::scrapers::scraper_resources::resources::RecentHtmlTarget;

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Recent {
	pub meta: Meta,
	pub warthunder_news: Value,
	pub warthunder_changelog: Value,
	pub forums_updates_information: Value,
	pub forums_project_news: Value,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Meta {
	pub timestamp: u64,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Value {
	pub domain: String,
	pub selector: String,
	pub pin: String,
	pub recent_url: Vec<String>,
}

impl Value {
	pub fn is_outdated(&self, value: &str) -> bool {
		if self.recent_url.contains(&value.to_owned()) {
			println!("Content was recently fetched and is not new");
			info!("Content was recently fetched and is not new");
			false
		} else {
			println!("New post found, hooking now");
			warn!("New post found, hooking now");
			true
		}
	}
}

impl Recent {
	pub fn append_latest_warthunder_news(&mut self, value: &str) {
		self.warthunder_news.recent_url.push(value.to_owned());
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_changelog(&mut self, value: &str) {
		self.warthunder_changelog.recent_url.push(value.to_owned());
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_forums_updates_information(&mut self, value: &str) {
		self.forums_updates_information.recent_url.push(value.to_owned());
		self.write_latest(value);
	}
	pub fn append_latest_warthunder_forums_project_news(&mut self, value: &str) {
		self.forums_project_news.recent_url.push(value.to_owned());
		self.write_latest(value);
	}
	pub fn read_latest() -> Self {
		let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
		let recent: Self = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");
		recent
	}
	fn write_latest(&self, value: &str) {
		let write = serde_json::to_string_pretty(self).unwrap();
		fs::write("assets/recent.json", write).expect("Couldn't write to recent file");
		println!("Written {} to file", value);
		warn!("Written {} to file", value);
	}
}

pub fn format_selector(main: &Value, which: &RecentHtmlTarget, index: u32) -> Selector {
	return match which {
		RecentHtmlTarget::Pin => {
			Selector::parse(&*format!("{}{}{}", &*main.pin.split_whitespace().collect::<Vec<&str>>()[0], index, &*main.pin.split_whitespace().collect::<Vec<&str>>()[1])).unwrap()
		}
		RecentHtmlTarget::Post => {
			Selector::parse(&*format!("{}{}{}", &*main.selector.split_whitespace().collect::<Vec<&str>>()[0], index, &*main.selector.split_whitespace().collect::<Vec<&str>>()[1])).unwrap()
		}
	};
}
