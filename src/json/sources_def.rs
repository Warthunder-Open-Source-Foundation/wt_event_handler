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
					scrape_type: ScrapeType::Main,
					tracked_urls: HashMap::new()
				},
				Source {
					name: "warthunder_changelog".to_owned(),
					domain: "https://warthunder.com/en/game/changelog/".to_owned(),
					scrape_type: ScrapeType::Changelog,
					tracked_urls: HashMap::new()
				},
				Source {
					name: "forums_updates_information".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/126-updates-information-read-only/&ct=1626882238".to_owned(),
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new()
				},
				Source {
					name: "forums_project_news".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/26-project-news-read-only/&ct=1630343851".to_owned(),
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new()
				},
				Source {
					name: "forums_notice_board".to_owned(),
					domain: "https://forum.warthunder.com/index.php?/forum/1500-notice-board-announcements-information/".to_owned(),
					scrape_type: ScrapeType::Forum,
					tracked_urls: HashMap::new()
				}
			]
		}
	}
}