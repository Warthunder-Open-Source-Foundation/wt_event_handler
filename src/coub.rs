use std::{fs, mem};

use reqwest::get;
use scraper::{Html, Selector};
use serde_json;

pub async fn html_processor_coub() -> String {
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
    let mut cache: Root = serde_json::from_str(&cache_raw).expect("Json cannot be read");

    let url = &cache.targets[1].domain;


    println!("Fetching data from {}", url);
    let html = Html::parse_document(&get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap());
    println!("Fetched data with size of {} bytes", mem::size_of_val(&html));

    // println!("{:?}", html);

    let top_url_selector = Selector::parse("body > div.body-container > div.logged.page-container > div.page__content > div.page__body.grid-container.gutter-gamma > div > div.coubs-list > div.coubs-list__inner.masonry > div.page > div:nth-child(1) > div.coub__inner > div > div.coub__description > div > div.description__info > h5 > a").unwrap();

    let top_url = html.select(&top_url_selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    println!("{}", top_url);

    "yes".to_string()
}