use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Timeout {
	pub blocked: HashMap<String, i64>,
}

impl Timeout {
	pub fn new() -> Self {
		Self {
			blocked: HashMap::new(),
		}
	}
	pub fn time_out(&mut self, source: String, until: i64) {
		self.blocked.insert(source, until);
	}
	pub fn is_timed_out(&self, source: &str) -> bool {
		return if let Some(time) = self.blocked.get(source) {
			let now = chrono::Utc::now().timestamp();

			now < *time
		} else {
			false
		}
	}
}