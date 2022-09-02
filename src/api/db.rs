use sqlx::{ConnectOptions, SqliteConnection};
use std::str::FromStr;
use sqlx::sqlite::SqliteConnectOptions;
use crate::api::db_error::DatabaseError;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./assets");

pub struct Database {
	pub connection: SqliteConnection,
}

impl Database {
	pub async fn new() -> Result<Self, DatabaseError> {
		let mut db = SqliteConnectOptions::from_str("sqlite::memory:")?
			.connect().await?;

		MIGRATOR.run(&mut db).await?;

		Ok(Self {
			connection: db
		})
	}
}