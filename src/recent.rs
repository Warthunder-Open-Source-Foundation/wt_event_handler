#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Root {
	pub targets: Vec<Target>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Target {
	pub name: String,
	pub recent_url: String,
	pub domain: String,
}
