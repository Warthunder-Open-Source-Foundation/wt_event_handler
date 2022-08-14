use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

use chrono::{Month, NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::Client;
use scraper::{Html, Selector};

use crate::{LogLevel, print_log};
use crate::error::NewsError;
use crate::error::NewsError::{BadSelector, MonthParse, SelectedNothing};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ScrapeType {
	Forum,
	Main,
	Changelog,
}

impl Display for ScrapeType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ScrapeType::Forum => {
				write!(f, "Forum news")
			}
			ScrapeType::Main => {
				write!(f, "News")
			}
			ScrapeType::Changelog => {
				write!(f, "Changelog")
			}
		}
	}
}

pub async fn request_html(url: &str) -> Result<Html, Box<dyn Error>> {
	print_log(&format!("Fetching data from {}", &url), LogLevel::Info);

	let client = Client::builder()
		.timeout(Duration::from_secs(1))
		.build()?;
	let raw_html = client.get(url).send().await?;
	let text = raw_html.text().await?;
	Ok(Html::parse_document(text.as_str()))
}

pub fn get_listed_links(scrape_type: ScrapeType, html: &Html) -> Result<Vec<(String, i64)>, Box<dyn Error>> {
	return match scrape_type {
		ScrapeType::Changelog | ScrapeType::Main => {
			let sel_text = match scrape_type {
				ScrapeType::Main => {
					// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
					"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(2) > div > section > div > div.showcase__content-wrapper > div.showcase__item"
				}
				ScrapeType::Changelog => {
					// ---------------------------------------------------------↓ I dont make the rules ¯\_(ツ)_/¯
					"#bodyRoot > div.content > div:nth-child(2) > div:nth-child(3) > div > section > div > div.showcase__content-wrapper > div.showcase__item"
				}
				_ => {
					panic!("Impossible")
				}
			};
			let sel = Selector::parse(sel_text).map_err(|_| NewsError::BadSelector(sel_text.to_owned()))?;

			const DATE_SEL_TEXT: &str = "div.widget__content > ul > li";
			let date_sel = Selector::parse(DATE_SEL_TEXT).map_err(|_| NewsError::BadSelector(DATE_SEL_TEXT.to_owned()))?;

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				let date_elem = item.select(&date_sel).next().ok_or(SelectedNothing(DATE_SEL_TEXT.to_owned(), item.inner_html()))?;
				let date_str = date_elem.inner_html().trim().to_owned();
				let split = date_str.split(" ").collect::<Vec<&str>>();
				let date = NaiveDate::from_ymd(i32::from_str(split[2])?, Month::from_str(split[1]).or_else(|_| Err(MonthParse(split[1].to_owned())))?.number_from_month(), u32::from_str(split[0])?).and_time(NaiveTime::from_hms(0, 0, 0));
				if let Some(url) = item.select(&Selector::parse("a").map_err(|_| NewsError::BadSelector(sel_text.to_owned()))?).next().ok_or(SelectedNothing(DATE_SEL_TEXT.to_owned(), item.inner_html()))?.value().attr("href") {
					res.push((url.to_owned(), date.timestamp()));
				}
			}
			Ok(res)
		}
		ScrapeType::Forum => {
			const SEL_TEXT: &str = "body > main > div > div > div > div:nth-child(2) > div > ol > li";
			let sel = Selector::parse(SEL_TEXT).map_err(|_| BadSelector(SEL_TEXT.to_owned()))?;

			const LOWER_URL_TEST: &str = "div > h4 > div > a";
			let lower_url = Selector::parse(LOWER_URL_TEST).map_err(|_| BadSelector(LOWER_URL_TEST.to_owned()))?;

			const DATE_SEL_TEXT: &str = "div > div > time";
			let date_sel = Selector::parse(DATE_SEL_TEXT).map_err(|_| NewsError::BadSelector(DATE_SEL_TEXT.to_owned()))?;

			let selected = html.select(&sel);
			let mut res = vec![];
			for item in selected {
				if let Some(url_elem) = item.select(&lower_url).next() {
					if let Some(url) = url_elem.value().attr("href") {
						if let Some(date_str) = item.select(&date_sel).next().ok_or(SelectedNothing(DATE_SEL_TEXT.to_owned(), item.inner_html()))?.value().attr("datetime").to_owned() {
							let date = NaiveDateTime::parse_from_str(&date_str.replace("Z", "").replace("T", ""), "%Y-%m-%d %H:%M:%S")?;
							res.push((url.to_owned(), date.timestamp()));
						}
					}
				}
			}
			Ok(res)
		}
	};
}

pub fn format_result(top_url: &str, selection: ScrapeType) -> String {
	return match selection {
		ScrapeType::Main | ScrapeType::Changelog => {
			format!("https://warthunder.com{}", top_url)
		}
		ScrapeType::Forum => {
			top_url.to_owned()
		}
	};
}