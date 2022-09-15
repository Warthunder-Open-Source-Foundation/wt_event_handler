use scraper::{Html, Selector};
use scraper::node::Element;

use crate::NewsError;

/// Takes simple selector and returns element from it
pub fn select_first(html: &Html, selector: &str, url: &str) -> Result<Element, NewsError> {
	if let Some(selected) = html.select(&format_selector(selector)?).next() {
		Ok(selected.value().to_owned())
	} else {
		Err(NewsError::SelectedNothing(selector.to_owned(), url.to_owned()))
	}
}

/// Takes element and returns attribute
pub fn select_attribute(elem: &Element, attr: &str, url: &str) -> Result<String, NewsError> {
	if let Some(val) = elem.attr(attr) {
		Ok(val.to_owned())
	} else {
		Err(NewsError::SelectedNothing(attr.to_owned(), url.to_owned()))
	}
}

// Formats selector, wraps around incompatible error coming from cssparser
pub fn format_selector(sel_text: &str) -> Result<Selector, NewsError> {
	match Selector::parse(sel_text) {
		Ok(selector) => Ok(selector),
		Err(error) => {
			Err(NewsError::BadSelector(sel_text.to_owned()))
		}
	}
}