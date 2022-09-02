use std::str::FromStr;
use sqlx::sqlite::SqliteConnectOptions;

pub fn start_db() {
	let db = SqliteConnectOptions::from_str("sqlite::memory:")
		.unwrap();
}

// async fn update_json(&self) -> Result<(), NewsError> {
// 	let json_value = match serde_json::to_value(&self)? {
// 		Value::Object(mut map) => {
// 			let tracked_urls = self.tracked_urls.read().await;
// 			map.insert("tracked_urls".to_owned(), serde_json::to_value(&*tracked_urls)?);
//
// 			serde_json::to_value(map)?
// 		}
// 		_ => { unreachable!() } // unreachable because we know it can't happen. We just passed in a struct ("Object") to get the value
// 	};
// 	let json = serde_json::to_string(&json_value)?;
//
// 	{
// 		*self.json.write().await = json;
// 	}
//
// 	Ok(())
// }