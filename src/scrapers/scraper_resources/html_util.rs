use scraper::{ElementRef, Html, Selector};
use scraper::node::Element;

use crate::NewsError;

pub trait HtmlUtil {
	/// Takes simple selector and returns element from it
	fn select_first(&self, selector: impl IntoSelector, url: &str) -> Result<Element, NewsError>;
}

impl HtmlUtil for Html {
	fn select_first(&self, selector: impl IntoSelector, url: &str) -> Result<Element, NewsError> {
		let parsed = selector.into_selector()?;
		if let Some(selected) = self.select(&parsed.sel).next() {
			Ok(selected.value().clone())
		} else {
			Err(NewsError::SelectedNothing(parsed.css_text, url.to_owned()))
		}
	}
}

impl HtmlUtil for ElementRef<'_> {
	fn select_first(&self, selector: impl IntoSelector, url: &str) -> Result<Element, NewsError> {
		let parsed = selector.into_selector()?;
		if let Some(selected) = self.select(&parsed.sel).next() {
			Ok(selected.value().clone())
		} else {
			Err(NewsError::SelectedNothing(parsed.css_text, url.to_owned()))
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
		Err(_) => {
			Err(NewsError::BadSelector(sel_text.to_owned()))
		}
	}
}

pub struct SelectorWrapper {
	sel: Selector,
	css_text: String,
}

impl SelectorWrapper {
	pub fn new(sel_text: &str) -> Result<Self, NewsError> {
		Ok(Self {
			sel: format_selector(sel_text)?,
			css_text: sel_text.to_owned(),
		})
	}
}

pub trait IntoSelector {
	fn into_selector(self) -> Result<SelectorWrapper, NewsError>;
}

impl IntoSelector for &str {
	fn into_selector(self) -> Result<SelectorWrapper, NewsError> {
		SelectorWrapper::new(self)
	}
}