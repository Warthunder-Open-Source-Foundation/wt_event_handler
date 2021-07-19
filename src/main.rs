use reqwest::get;
use scraper::{Html, Selector};
use serenity::client::ClientBuilder;
use serenity::prelude::{EventHandler, Context};
use serenity::model::id::ChannelId;
use std::fs::read_to_string;
use serenity::model::prelude::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;

async fn main() {

    let token = read_to_string("/home/flareflo/CLionProjects/WT_event_handler/assets/discord_token.txt").unwrap();

    let mut client = ClientBuilder::new(token).event_handler(Handler).await.unwrap();

    client.start().await.unwrap();


}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} fucked your mom", data_about_bot.user.name);
        secondary(ctx).await;
    }
}

#[tokio::main]
async fn secondary(ctx: Context) {
    let html = Html::parse_document(&get("https://warthunder.com/en/news/").await.unwrap().text().await.unwrap());
    let top_article_selector = "#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1)";
    let top_article = html.select(&Selector::parse(top_article_selector).unwrap()).next().unwrap().text().collect::<String>();

    ChannelId::from(866634236232597534).say(&ctx.http, top_article).await.unwrap();
}