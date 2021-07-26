#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookAuth {
	pub hooks: Vec<Hooks>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Hooks {
	pub name: String,
	pub token: String,
	pub uid: u64,
	pub filter: FilterType,
	pub keywords: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FilterType {
	Default = 0,
	Blacklist = 1,
	Whitelist = 2,
}
impl Default for FilterType {
	fn default() -> Self {
		Self::Default
	}
}