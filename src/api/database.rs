use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;

use sqlx::{Executor, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;

use crate::api::db_error::DatabaseError;

#[derive(Clone)]
pub struct Database {
	pub(crate) connection: SqlitePool,
	pub(crate) latest_timestamp: Arc<AtomicI64>,
}


// ONLY IMPL DATABASE INTERNAL THINGS HERE, API ORIENTED FUNCTIONALITY GOES INTO database_queries.rs
impl Database {
	pub async fn new() -> Result<Self, DatabaseError> {
		let options = SqliteConnectOptions::from_str("sqlite::memory:")?
			.create_if_missing(true);
		let db = SqlitePool::connect_with(options).await?;

		db.execute(include_str!("../../assets/setup_db.sql")).await?;

		Ok(Self {
			connection: db,
			latest_timestamp: Arc::new(AtomicI64::new(0)),
		})
	}
}