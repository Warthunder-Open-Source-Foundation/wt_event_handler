use std::str::FromStr;
use sqlx::sqlite::SqliteConnectOptions;

pub fn start_db() {
	let db = SqliteConnectOptions::from_str("sqlite::memory:")
		.unwrap();
}