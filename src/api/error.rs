use std::fmt::{Debug, Display};
use actix_web::ResponseError;
use thiserror::Error as ThisError;
use crate::NewsError;

#[derive(Debug, ThisError)]
pub enum ApiError {
	#[error(transparent)]
	InternalServerError(#[from] NewsError),
}


impl ResponseError for ApiError {}