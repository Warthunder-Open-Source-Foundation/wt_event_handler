#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Recent {
	pub targets: Vec<Target>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Target {
	pub name: String,
	pub recent_url: Vec<String>,
	pub domain: String,
}
