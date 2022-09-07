use std::process::exit;
use actix_web::{get, web, Responder};
use crate::api::database::Database;
use crate::json::sources::Sources;

#[get("/news/latest/{source}")]
#[allow(clippy::unused_async)]
pub async fn greet(source: web::Path<String>, db: web::Data<Database>) -> impl Responder {
	db.get_latest_news_from_source(&*source).await.unwrap().to_owned()

}

#[get("/news/latest")]
#[allow(clippy::unused_async)]
pub async fn get_latest_news(db: web::Data<Database>) -> impl Responder {
	let mut total = vec![];
	for source in Sources::new().sources {
		total.push(db.get_latest_news_from_source(&source.name).await.unwrap())
	}
	serde_json::to_string(&total).unwrap()
}


#[get("/settings/shutdown")]
#[allow(clippy::unused_async)]
pub async fn shutdown() -> impl Responder {
	exit(1);
	#[allow(unreachable_code)]
	""
}