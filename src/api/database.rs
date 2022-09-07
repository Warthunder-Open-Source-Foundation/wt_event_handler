use sqlx::{ConnectOptions, Encode, Executor, Pool, query, query_file, query_file_as_unchecked, query_file_unchecked, Row, Sqlite, SqliteConnection, SqlitePool};
use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteRow};
use crate::api::db_error::DatabaseError;

use sqlx::migrate::Migrator;

#[derive(Clone)]
pub struct Database {
	pub connection: SqlitePool,
}


// ONLY IMPL DATABASE INTERNAL THINGS HERE, API ORIENTED FUNCTIONALITY GOES INTO database_queries.rs
impl Database {
	pub async fn new() -> Result<Self, DatabaseError> {
		let options = SqliteConnectOptions::from_str("sqlite::memory:")?
			.create_if_missing(true)
			.shared_cache(true)
			.journal_mode(SqliteJournalMode::Wal);
		let mut db = SqlitePool::connect_with(options).await?;

		db.execute(include_str!("../../assets/setup_db.sql")).await?;

		Ok(Self {
			connection: db
		})
	}
}