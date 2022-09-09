use sqlx::{Executor, query, Row};
use crate::api::database::Database;
use crate::api::db_error::DatabaseError;

impl Database {
	pub async fn store_recent_single(&self, value: &str, source: &str) -> Result<(), DatabaseError>
	{
		let now = chrono::Utc::now().timestamp();
		let q = query!("INSERT INTO sources (url, fetch_date, source)
						VALUES (?, ?, ?);",
						value, now, source);
		self.connection.execute(q).await?;
		Ok(())
	}

	pub async fn store_recent<I>(&self, values: I, source: &str) -> Result<(), DatabaseError>
		where I: IntoIterator,
			  I::Item: ToString
	{
		for value in values {
			self.store_recent_single(&value.to_string(), source).await?;
		}
		Ok(())
	}

	pub async fn get_latest_news_from_source(&self, source_name: &str) -> Result<String, DatabaseError> {
		let q = query!("SELECT url
						FROM sources
						WHERE source = ?
						ORDER BY fetch_date DESC", source_name);
		Ok(self.connection.fetch_one(q).await?.get(0))
	}

	pub async fn get_latest_timestamp(&self) -> Result<i64, DatabaseError> {
		let q = query!("SELECT  fetch_date
								 FROM sources
								 ORDER BY fetch_date DESC ");
		Ok(self.connection.fetch_one(q).await?.get(0))
	}
}