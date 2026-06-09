use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tower_sessions::session;

use crate::auth::error::AuthError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("session storage failed: {0}")]
    SessionError(#[from] session::Error),
    #[error("authorization error: {0}")]
    AuthError(#[from] AuthError),
    #[error("bad request: {0}")]
    BadRequest(#[from] RequestError),
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Missing session: {0}")]
    MissingSession(&'static str),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Self::SessionError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Session storage failed"),
            Self::AuthError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Authorization failed"),
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            Self::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database failure"),
        };

        body.into_response()
    }
}
