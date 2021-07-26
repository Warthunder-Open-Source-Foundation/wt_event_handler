use std::fs;

use log::*;

pub fn convert(name: &str) -> usize {
	#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
	pub struct Root {
		pub targets: Vec<Target>,
	}

	#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
	pub struct Target {
		pub name: String,
		pub recent_url: String,
		pub domain: String,
	}

	let cache_raw = fs::read_to_string("recent.json").expect("Cannot read file");
	let cache: Root = serde_json::from_str(&cache_raw).expect("Json cannot be read");

	let index: Option<usize>;
	index = cache.targets.iter().position(|r| r.name == name);

	if index.is_none() {
		println!("Index could not be resolved");
		error!("Index could not be resolved");
		panic!();
	}

	return index.unwrap();
}