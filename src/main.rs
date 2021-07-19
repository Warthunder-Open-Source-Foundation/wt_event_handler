use std::fs;

use reqwest::{ClientBuilder, get};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    println!("Fetching data");

    let url = "https://warthunder.com/en/news/";
    let html = Html::parse_document(&get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap());
    println!("Fetched data");

    let top_article_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1)").unwrap();
    let top_url_selector = Selector::parse("#bodyRoot > div.content > div:nth-child(2) > div > div > section > div > div.showcase__content-wrapper > div:nth-child(1) > a").unwrap();

    let top_article = html.select(&top_article_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let top_url = html.select(&top_url_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();


    let top_article = top_article.replace("  ", "").replace("\n\n", "");
    let top_url = format!("https://warthunder.com{}", top_url);

    println!("Data fetched; {}", top_article);
    println!("With URL; {}", top_url);

    check(&top_url);

    fn check(url: &str) {
        let keywords = vec!["devblog", "event", "maintenance", "major", "trailer", "teaser", "developers", "fixed", "shooting"];
        for keyword in keywords {
            if url.contains(keyword) {
                println!("URL {} matched with keyword {}", url, keyword);
                fs::write("url_socket.txt", url);
                break;
            }
        }
    }
}
