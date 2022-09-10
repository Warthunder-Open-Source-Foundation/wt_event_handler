use std::str::FromStr;
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use crate::api::db_error::DatabaseError;

use sqlx::{Executor, SqlitePool};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
	pub connection: SqlitePool,
	pub(crate) latest_timestamp: Arc<Mutex<(i64, String)>>,
}


// ONLY IMPL DATABASE INTERNAL THINGS HERE, API ORIENTED FUNCTIONALITY GOES INTO database_queries.rs
impl Database {
	pub async fn new() -> Result<Self, DatabaseError> {
		let options = SqliteConnectOptions::from_str("sqlite::memory:")?
			.create_if_missing(true)
			.shared_cache(true)
			.journal_mode(SqliteJournalMode::Wal);
		let db = SqlitePool::connect_with(options).await?;

		db.execute(include_str!("../../assets/setup_db.sql")).await?;

		Ok(Self {
			connection: db,
			latest_timestamp: Arc::new(Mutex::new((0, "".to_string())))
		})
	}
}