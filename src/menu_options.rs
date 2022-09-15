use std::fs;
use std::io;
use std::process::exit;
use std::str::FromStr;

use crate::{NewsError, TOKEN_PATH};
use crate::embed::EmbedData;
use crate::json::webhooks::{Hooks, WebhookAuth};
use crate::webhook_handler::deliver_webhook;

pub async fn add_webhook() -> Result<(), NewsError> {
	let token_raw = fs::read_to_string(TOKEN_PATH)?;
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw)?;

	webhook_auth.hooks.push(Hooks::from_user().await);

	let write = serde_json::to_string_pretty(&webhook_auth)?;
	fs::write(TOKEN_PATH, write)?;
	exit(0);
}

pub async fn test_hook() -> Result<(), NewsError> {
	let mut line = String::new();

	println!("Choose the webhook order in the array to test\n");

	io::stdin().read_line(&mut line)?;

	let pos = usize::from_str(line.trim()).expect("Expected integer");

	deliver_webhook(EmbedData::test(), pos).await;

	exit(0);
}

pub fn remove_webhook() -> Result<(), NewsError> {
	let token_raw = fs::read_to_string(TOKEN_PATH)?;
	let mut webhook_auth: WebhookAuth = serde_json::from_str(&token_raw)?;
	let mut line = String::new();

	println!("These are the following available webhooks");
	for (i, hook) in webhook_auth.hooks.iter().enumerate() {
		println!("{} {}", i, hook.name);
	}
	println!("Choose the webhook to remove \n");

	io::stdin().read_line(&mut line)?;
	let index = line.trim().parse().expect("Expected integer");

	webhook_auth.hooks.remove(index);

	let write = serde_json::to_string_pretty(&webhook_auth)?;
	fs::write(TOKEN_PATH, write)?;

	println!("Webhook {} successfully removed", index);
	exit(0);
}