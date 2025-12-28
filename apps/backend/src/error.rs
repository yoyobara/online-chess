use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use crate::repositories::user::UserRepositoryError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("could not find user")]
    UserNotFound,

    #[error("the field {0} already exists")]
    EmailOrNameExists(String),

    #[error("password and email do not match")]
    WrongPassword,

    // TODO remove for security
    #[error("something went wrong: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            ApiError::UserNotFound => StatusCode::NOT_FOUND,
            ApiError::EmailOrNameExists(_) => StatusCode::CONFLICT,
            ApiError::WrongPassword => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({"error": self.to_string() });

        (status_code, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

// repos conversion
impl From<UserRepositoryError> for ApiError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::UserNotFound => Self::UserNotFound,
            UserRepositoryError::ConstraintViolation(field) => Self::EmailOrNameExists(field),
            UserRepositoryError::Db(err) => Self::InternalServerError(err),
        }
    }
}
