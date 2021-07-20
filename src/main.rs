use std::fs;
use std::thread::sleep;
use std::time;

use rand;
use rand::Rng;
use reqwest::get;
use scraper::{Html, Selector};
use serenity;
use serenity::http::client::Http;
// use serenity::model::id::{ChannelId, GuildId, WebhookId};
// use serenity::model::prelude::{WebhookType};

// use serenity::client::{ClientBuilder, EventHandler};
// use serenity::model::channel::{Message, Embed};

#[tokio::main]
async fn main() {
    loop {
        handle_webhook().await;
        let wait = rand::thread_rng().gen_range(50..70);
        println!("Waiting for {} seconds", wait);
        sleep(time::Duration::from_secs(wait))
    }
    async fn handle_webhook() {
        // Using environmental variable due to some compatibility issues
        // TODO comment lower line and add variable
        let token = fs::read_to_string("assets/discord_token.txt").unwrap();
        // TODO uncomment lower line and add variable
        // let token= "";

        let id = 867052162970288159;

        let my_http_client = Http::new_with_token(&token);

        println!("{} {}", id, &token);
        let webhook = match my_http_client.get_webhook_with_token(id, token.as_str()).await {
            Err(why) => {
                println!("{}", why);
                panic!("")
            }
            Ok(hook) => hook,
        };

        let content = html_procecss().await;
        // let embed = Embed::fake(|mut e| {
        //     // e.title("Cool news and that shit");
        //     // e.description("Very nice");
        //     e.url(content);
        //     e
        // });

        let recent = fs::read_to_string("recent.txt").expect("Cannot read file");

        if !content.contains("No match found") {
            if recent != content {
                println!("New post found, hooking now");
                webhook.execute(&my_http_client, false, |w| {
                    fs::write("recent.txt", &content).expect("Writing to recent file failed");
                    w.content(&format!("[{a}]({a})", a = content));
                    w.username("The WT news bot");
                    // w.embeds(vec![embed]);
                    w
                })
                    .await
                    .unwrap();
            }else {
                println!("Content was recently fetched and is not new");
            }
        } else {
            println!("Content was either not a match")
        }
    }

    async fn html_procecss() -> String {
        println!("Fetching data");

        let url = "https://warthunder.com/en/news/";
        let html = Html::parse_document(&get(url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap());
        println!("Fetched data");

        // let top_article_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1)").unwrap();
        let top_url_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap();

        // let top_article = html.select(&top_article_selector)
        //     .next()
        //     .unwrap()
        //     .text()
        //     .collect::<String>();
        let top_url = html.select(&top_url_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();


        // let top_article = top_article.replace("  ", "").replace("\n\n", "");
        let keywords = vec![
            "devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
            "fixed", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
            "shop", "special"
        ];
        let top_url = &*format!("https://warthunder.com{}", top_url);

        for keyword in keywords {
            if top_url.contains(keyword) {
                println!("URL {} matched with keyword {}", top_url, keyword);
                return (top_url).parse().unwrap();
            }
        }
        return String::from("No match found");
    }
}