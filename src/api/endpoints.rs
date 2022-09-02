use actix_web::{get, web, Responder};
use crate::json::sources::Sources;

#[get("/news/latest/{source}")]
#[allow(clippy::unused_async)]
pub async fn greet(source: web::Path<String>) -> impl Responder {
	format!("Hello {source}!")
}

#[get("/news/latest")]
#[allow(clippy::unused_async)]
pub async fn get_latest_news(sources: web::Data<Sources>) -> impl Responder {
	serde_json::to_string(&sources.get_latest().await)
}