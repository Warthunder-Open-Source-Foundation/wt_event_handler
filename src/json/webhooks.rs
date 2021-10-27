use std::io;
use std::process::exit;

use log::error;
use serenity::http::Http;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct WebhookAuth {
	pub hooks: Vec<Hooks>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Hooks {
	pub name: String,
	pub token: String,
	pub uid: u64,
	pub main_filter: FilterType,
	pub forum_filter: FilterType,
	pub main_keywords: Vec<String>,
	pub forum_keywords: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
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
	pub async fn from_user() -> Self {
		let mut val = Self {
			name: "".to_string(),
			token: "".to_string(),
			uid: 0,
			main_filter: FilterType::default(),
			forum_filter: FilterType::default(),
			main_keywords: vec![],
			forum_keywords: vec![]
		};
		let mut line = String::new();

		println!("Enter the Name for the webhook (you can always abort with n) \n");
		io::stdin().read_line(&mut line).unwrap();

		if let "n" = line.trim() {
			println!("Aborting webhook creation");
			exit(0);
		}

		val.name = line.clone();
		val.name.pop();

		println!("Enter the URL for the webhook \n");
		line.clear();
		io::stdin().read_line(&mut line).unwrap();

		if let "n" = line.trim() {
			println!("Aborting webhook creation");
			exit(0);
		}

		line.pop();
		let uid_token: Vec<String> = line.split('/').map(String::from).collect();
		val.uid = uid_token[5].parse().unwrap();
		val.token = uid_token[6].clone();

		println!("Choose a filter option the main and forum list seperated by 1 space (such as \"1 2\"): \n 1. Default \n 2. Blacklist \n 3. Whitelist  \n");
		line.clear();
		io::stdin().read_line(&mut line).unwrap();

		if let "n" = line.trim() {
			println!("Aborting webhook creation");
			exit(0);
		}

		let option = line.clone();
		let main_filter_string = option.split_at(1).0;
		let forum_filter_string = option.split_at(1).1;
		val.main_filter = FilterType::from_user(main_filter_string);
		val.forum_filter = FilterType::from_user(forum_filter_string);

		if val.main_filter != FilterType::Default {
			let mut line = String::new();
			println!("Enter main keywords, seperated by spaces all lowercase");
			line.clear();
			io::stdin().read_line(&mut line).unwrap();
			val.main_keywords = line.split_whitespace().map(String::from).collect();
		}

		if val.forum_filter != FilterType::Default {
			let mut line = String::new();
			println!("Enter forum keywords, seperated by spaces all lowercase");
			line.clear();
			io::stdin().read_line(&mut line).unwrap();
			val.forum_keywords = line.split_whitespace().map(String::from).collect();
		}


		println!("Entry created successfully, do you want to send a test-message to test the hook? y/n \n");
		line.clear();
		io::stdin().read_line(&mut line).unwrap();
		match line.trim() {
			"y" => {
				send_test_hook(&val).await;
			}
			"n" => {}
			_ => {
				println!("No option specified");
				exit(1);
			}
		};
		val
	}
}

async fn send_test_hook(hook: &Hooks) {
	let token = &hook.token;
	let uid = &hook.uid;

	let my_http_client = Http::new_with_token(token);

	let webhook = match my_http_client.get_webhook_with_token(*uid, token).await {
		Err(why) => {
			println!("{}", why);
			error!("{}", why);
			panic!("")
		}
		Ok(hook) => hook,
	};


	webhook.execute(my_http_client, false, |w| {
		w.content(&format!("Webhook {} was successfully created", &hook.name));
		w.username("The WT news bot");
		w.avatar_url("https://cdn.discordapp.com/attachments/866634236232597534/868623209631744000/the_news_broke.png");
		w
	})
		.await
		.unwrap();
}