use std::collections::HashMap;

use crate::json::sources::{Source, Sources};
use crate::scrapers::scraper_resources::resources::ScrapeType;

impl Sources {
	pub(crate) fn new() -> Self {
		Self {
			sources: vec![
				Source {
					name: "warthunder_news".to_owned(),
					domain: "https://warthunder.com/en/news".to_owned(),
					id: 0,
					scrape_type: ScrapeType::Main,
					tracked_urls: HashMap::new(),
				},
				Source {
					name: "warthunder_changelog".to_owned(),
					domain: "https://warthunder.com/en/game/changelog/".to_owned(),
					id: 1,
					scrape_type: ScrapeType::Changelog,
					tracked_urls: HashMap::new(),
				},
				Source {
					name: "forums_updates_information".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/126-updates-information-read-only/&ct=1626882238".to_owned(),
					id: 2,
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new(),
				},
				Source {
					name: "forums_project_news".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/26-project-news-read-only/&ct=1630343851".to_owned(),
					id: 3,
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new(),
				},
				Source {
					name: "forums_notice_board".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/1500-notice-board-announcements-information/".to_owned(),
					id: 4,
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new(),
				},
			]
		}
	}
	pub fn id_from_name(name: &str) -> u8 {
		#[allow(clippy::match_same_arms)]
		match name {
			"warthunder_news" => 0,
			"warthunder_changelog" => 1,
			"forums_updates_information" => 2,
			"forums_project_news" => 3,
			"forums_notice_board" => 4,
			_ => 0,
		}
	}
}