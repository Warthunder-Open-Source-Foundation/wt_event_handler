use std::process::exit;
use actix_web::{get, web, Responder};
use actix_web::error::{ErrorForbidden, ErrorGone, ErrorUnauthorized};
use actix_web::http::StatusCode;
use crate::api::database::Database;
use crate::error::{error_webhook, ship_error_webhook};
use crate::json::sources::Sources;
use crate::SHUTDOWN_KEY;

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

#[get("/settings/shutdown/{key}")]
#[allow(clippy::unused_async)]
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

#[get("/news/timestamp")]
#[allow(clippy::unused_async)]
pub async fn get_latest_timestamp(db: web::Data<Database>) -> impl Responder {
	db.get_latest_timestamp().await.expect("uh").to_string()
}