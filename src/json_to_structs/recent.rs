use std::fs;
use log::*;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Recent {
	pub warthunder_news: Target,
	pub warthunder_changelog: Target,
	pub forums: Target,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Target {
	pub recent_url: Vec<String>,
	pub domain: String,
}

impl Target {
	pub fn is_outdated(&self, value: &String) -> bool {
		if self.recent_url.last().unwrap() != value {
			println!("New post found, hooking now");
			warn!("New post found, hooking now");
			true
		} else {
			println!("Content was recently fetched and is not new");
			info!("Content was recently fetched and is not new");
			false
		}
	}
}

impl Recent {
	pub fn append_latest_warthunder_news(&mut self, value: &String) {
		self.warthunder_news.recent_url.push(value.clone());
		self.write_latest(&value);
	}
	pub fn append_latest_warthunder_changelog(&mut self, value: &String) {
		self.warthunder_changelog.recent_url.push(value.clone());
		self.write_latest(&value);
	}
	pub fn append_latest_warthunder_forums(&mut self, value: &String) {
		self.forums.recent_url.push(value.clone());
		self.write_latest(&value);
	}
	pub fn read_latest() -> Self {
		let cache_raw_recent = fs::read_to_string("assets/recent.json").expect("Cannot read file");
		let mut recent: Self = serde_json::from_str(&cache_raw_recent).expect("Json cannot be read");
		return recent;
	}
	fn write_latest(&self, value: &String) {
		let write = serde_json::to_string_pretty(self).unwrap();
		fs::write("assets/recent.json", write).expect("Couldn't write to recent file");
		println!("Written {} to file", value);
		warn!("Written {} to file", value);
	}
}
