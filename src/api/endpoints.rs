#![allow(clippy::unused_async)]

use std::process::exit;

use actix_web::{get, post, Responder, web};
use actix_web::error::{ErrorForbidden, ErrorGone};
use serde::{Deserialize, Serialize};

use crate::{BOOT_TIME, NewsError, SHUTDOWN_KEY};
use crate::api::database::Database;
use crate::api::error::ApiError;
use crate::error::ship_error_webhook;
use crate::json::sources::Sources;
use crate::scrapers::html_processing::get_embed_data;
use crate::scrapers::scraper_resources::resources::ScrapeType;

#[get("/news/latest/{source}")]
pub async fn greet(source: web::Path<String>, db: web::Data<Database>) -> impl Responder {
	db.get_latest_news_from_source(Sources::id_from_name(&source)).await.unwrap()
}

#[get("/news/latest")]
pub async fn get_latest_news(db: web::Data<Database>) -> impl Responder {
	let mut total = vec![];
	for source in Sources::new().sources {
		total.push(db.get_latest_news_from_source(source.id).await.unwrap());
	}
	serde_json::to_string(&total).unwrap()
}

#[get("/settings/shutdown/{key}")]
pub async fn shutdown(key: web::Path<String>) -> impl Responder {
	if *key == *SHUTDOWN_KEY {
		ship_error_webhook("A remote shutdown request was sent".to_owned(), &format!("Shutdown-key {key} was authenticated"), false).await;
		exit(1);
		#[allow(unreachable_code)]
		ErrorGone("Shutdown should be completed by now, if you read this, contact the person that gave you this key.")
	} else {
		ErrorForbidden("Bad shutdown-key")
	}.error_response()
}

// Does around 200 thousand requests per second, poll this
#[get("/news/timestamp")]
pub async fn get_latest_timestamp(db: web::Data<Database>) -> impl Responder {
	db.get_latest_timestamp().to_string()
}

#[get("/statistics/uptime")]
pub async fn get_uptime() -> impl Responder {
	BOOT_TIME.elapsed().as_secs().to_string()
}

#[derive(Deserialize, Serialize)]
pub struct ManualPost {
	pub save_to_db: bool,
	pub url: String,
}

#[post("/news/post")]
pub async fn post_manual(post: web::Json<ManualPost>) -> impl Responder {
	let scrape_type = ScrapeType::infer_from_url(&post.url);
	let embed = get_embed_data(&post.url, scrape_type).await?;
	embed.handle_webhooks(true, scrape_type).await;
	Ok::<&str, ApiError>("")
}