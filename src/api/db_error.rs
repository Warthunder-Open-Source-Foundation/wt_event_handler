use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
	#[error(transparent)]
	SqlxError(#[from] sqlx::Error),

	#[error(transparent)]
	MigrateError(#[from] sqlx::migrate::MigrateError),
}