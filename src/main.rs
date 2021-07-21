use std::fs;
use std::thread::sleep;
use std::time;

use rand;
use rand::Rng;
use reqwest::get;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use serenity;
use serenity::http::client::Http;

use crate::wt_news::html_processor_wt_news;

mod wt_news;

#[tokio::main]
async fn main() {
    loop {
        let content = html_processor_wt_news().await;
        handle_webhook(content).await;
        let wait = rand::thread_rng().gen_range(50..70);
        println!("Waiting for {} seconds", wait);
        sleep(time::Duration::from_secs(wait))
    }
    async fn handle_webhook(content: String) {
        // Using environmental variable due to some compatibility issues
        // TODO comment lower line and add variable
        let token = fs::read_to_string("assets/discord_token.txt").unwrap();
        // TODO uncomment lower line and add variable
        // let token = "";

        let id = 867052162970288159;

        let my_http_client = Http::new_with_token(&token);

        let webhook = match my_http_client.get_webhook_with_token(id, &token).await {
            Err(why) => {
                println!("{}", why);
                panic!("")
            }
            Ok(hook) => hook,
        };


        // let embed = Embed::fake(|mut e| {
        //     // e.title("Cool news and that shit");
        //     // e.description("Very nice");
        //     e.url(content);
        //     e
        // });

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

        let index = 0;

        let cache_raw = fs::read_to_string("recent.json").expect("Cannot read file");
        let mut cache: Root = serde_json::from_str(&cache_raw).expect("Json cannot be read");
        println!("{}", cache.targets[index].name);

        if !content.contains("No match found") {
            if cache.targets[index].recent_url != content {
                println!("New post found, hooking now");
                webhook.execute(&my_http_client, false, |w| {
                    cache.targets[index].recent_url = content;
                    w.content(&format!("[{a}]({a})", a = content));
                    w.username("The WT news bot");
                    // w.embeds(vec![embed]);
                    w
                })
                    .await
                    .unwrap();
            } else {
                println!("Content was recently fetched and is not new");
            }
        } else {
            println!("Content was either not a match")
        }
    }
}