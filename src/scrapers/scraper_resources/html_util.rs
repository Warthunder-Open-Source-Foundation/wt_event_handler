use scraper::{Html, Selector};
use scraper::node::Element;

use crate::NewsError;

pub trait HtmlUtil {
	/// Takes simple selector and returns element from it
	fn select_first(&self, selector: &str, url: &str) -> Result<Element, NewsError>;
}

impl HtmlUtil for Html {
	fn select_first(&self, selector: &str, url: &str) -> Result<Element, NewsError> {
		if let Some(selected) = self.select(&format_selector(selector)?).next() {
			Ok(selected.value().to_owned())
		} else {
			Err(NewsError::SelectedNothing(selector.to_owned(), url.to_owned()))
		}
	}
}

pub trait ElemUtil {
	/// Takes element and returns attribute
	fn select_attribute(&self, attr: &str, url: &str) -> Result<String, NewsError>;
}

impl ElemUtil for Element {
	fn select_attribute(&self, attr: &str, url: &str) -> Result<String, NewsError> {
		if let Some(val) = self.attr(attr) {
			Ok(val.to_owned())
		} else {
			Err(NewsError::SelectedNothing(attr.to_owned(), url.to_owned()))
		}
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