use sqlx::{Executor, query, Row};
use crate::api::database::Database;
use crate::api::db_error::DatabaseError;

impl Database {
	pub async fn store_recent_single(&self, value: &str, source: u8) -> Result<(), DatabaseError>
	{
		let now = chrono::Utc::now().timestamp();
		let q = query!(// language=SQL
			"INSERT INTO sources (url, fetch_date, source)
			VALUES (?, ?, ?);",
						value, now, source);
		self.connection.execute(q).await?;
		Ok(())
	}

	pub async fn store_recent<I>(&self, values: I, source: u8) -> Result<(), DatabaseError>
		where I: IntoIterator,
			  I::Item: ToString
	{
		for value in values {
			self.store_recent_single(&value.to_string(), source).await?;
		}
		Ok(())
	}

	pub async fn get_latest_news_from_source(&self, source_id: u8) -> Result<String, DatabaseError> {
		let q = query!(// language=SQL
			"SELECT url
			FROM sources
			WHERE source = ?
			ORDER BY fetch_date DESC", source_id);
		Ok(self.connection.fetch_one(q).await?.get(0))
	}

	pub async fn get_all_latest_news(&self) -> Result<Vec<String>, DatabaseError> {
		let q = query!(// language=SQL
			"SELECT url
			FROM sources
			GROUP BY source
			HAVING MAX(fetch_date) == fetch_date");
		Ok(self.connection.fetch_all(q).await?.into_iter().map(|x|x.get(0)).collect())
	}

	// Should prevent too many calls to DB when not directly required, not sure how smart this function is
	// Benchmarks show this runs around 2-3 times faster than querying the entire DB for timestamps
	pub async fn get_latest_timestamp(&self) -> Result<(i64, String), DatabaseError> {
		let ts = &mut *self.latest_timestamp.lock().await;
		// triggers if the latest timestamp is older than 10 seconds, otherwise the buffered result is returned
		if ts.0 < (chrono::Utc::now().timestamp() + 10000) {
			let latest = self.query_latest_timestamp().await?;
			ts.0 = latest.0;
			ts.1 = latest.1;
		}
		Ok(ts.clone())
	}

	async fn query_latest_timestamp(&self) -> Result<(i64, String), DatabaseError> {
		let q = query!(// language=SQL
			"SELECT fetch_date, source
			 FROM sources
			 ORDER BY fetch_date DESC ");
		let res = 	self.connection.fetch_one(q).await?;
		Ok((res.get(0), res.get(1)))
	}
}