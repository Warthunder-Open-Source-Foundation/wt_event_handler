use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Pointer};

#[derive(Debug, Clone)]
pub enum NewsError {
	NoUrlOnPost(String),
}

impl Display for NewsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			NewsError::NoUrlOnPost(err_message) => {
				write!(f, "No URL was scraped on target \'{err_message}\'")
			}
		}
	}
}

impl Error for NewsError {}