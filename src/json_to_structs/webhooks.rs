use std::io;
use std::process::exit;

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

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum FilterType {
	Default = 0,
	Blacklist = 1,
	Whitelist = 2,
}

impl FilterType {
	pub fn from_user(option: &str) -> Self {
		println!("{}", option);
		match option {
			"1" => {
				Self::Default
			}
			"2" => {
				Self::Blacklist
			}
			"3" => {
				Self::Whitelist
			}
			_ => {
				panic!("No option specified")
			}
		}
	}
}

impl Default for FilterType {
	fn default() -> Self {
		Self::Default
	}
}

impl Hooks {
	pub fn from_user() -> Self {
		let mut val = Self {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			filter: Default::default(),
			keywords: vec![]
		};
		let mut line = String::new();

		println!("Enter the Name for the webhook \n");
		io::stdin().read_line(&mut line).unwrap();
		val.name = line.clone();
		val.name.pop();

		println!("Enter the URL for the webhook \n");
		line.clear();
		io::stdin().read_line(&mut line).unwrap();
		line.pop();
		let uid_token: Vec<String> = line.split("/").map(|e| e.to_string()).collect();
		val.uid = uid_token[5].parse().unwrap();
		val.token = uid_token[6].clone();

		println!("Choose a filter option: \n 1. Default \n 2. Blacklist \n 3. Whitelist  \n");
		line.clear();
		io::stdin().read_line(&mut line).unwrap();
		let mut option = line.clone();
		option.pop();
		val.filter = FilterType::from_user(option.as_str());

		if val.filter != FilterType::Default {
			let mut line = String::new();
			println!("Enter the listing parameters, seperated by spaces all lowercase");
			line.clear();
			io::stdin().read_line(&mut line).unwrap();
			val.keywords = line.split_whitespace().map(|e| e.to_string()).collect();
		}
		val
	}
}