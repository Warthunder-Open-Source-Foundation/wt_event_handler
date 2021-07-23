use std::fs;

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


	// This is fucking retarded, hard coding a useless vector because iterating over it using length is not possible
	let iter = [0, 1, 2];
	let mut result: usize = usize::MAX;
	for targets in iter {
		if name == cache.targets[targets].name {
			result = targets;
		}
	}
	if result == usize::MAX {
		println!("Could not resolve index for {}", name);
		panic!();
	}
	return result;
}