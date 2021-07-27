use std::fs;

use log::*;
use crate::json_to_structs::recent::*;

pub fn convert(name: &str) -> usize {
	let cache_raw = fs::read_to_string("assets/recent.json").expect("Cannot read file");
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