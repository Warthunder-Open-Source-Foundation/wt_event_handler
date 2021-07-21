use scraper::{Html, Selector};
use reqwest::get;
use std::{fs, mem};

pub async fn html_processor_wt_news(index: usize) -> String {
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

    let url = &cache.targets[index].domain;

    println!("Fetching data from {}", url);
    let html = Html::parse_document(&get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap());
    println!("Fetched data with size of {} bytes", mem::size_of_val(&html));

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