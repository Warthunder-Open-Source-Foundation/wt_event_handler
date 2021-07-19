use reqwest::get;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let html = Html::parse_document(&get("https://warthunder.com/en/news/").await.unwrap().text().await.unwrap());
    let top_article_selector = "#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1)";
    let top_article = html.select(&Selector::parse(top_article_selector).unwrap()).next().unwrap().text().collect::<String>();

    if top_article.contains("Shooting") {
        println!("yes");
    }
}
