use actix_web::{get, web, App, HttpServer, Responder};
use crate::json::recent::Sources;
use tokio::sync::RwLock;

#[get("/news/latest/{source}")]
pub async fn greet(source: web::Path<String>) -> impl Responder {
	format!("Hello {source}!")
}

#[get("/news/latest")]
pub async fn get_latest_news(sources: web::Data<Sources>) -> impl Responder {
	format!{"{:#?}", sources}
}