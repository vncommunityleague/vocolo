use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::repository::{RepoError, RepoResult};

mod auth;
mod osu;
mod users;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn init_routes() -> Router {
    Router::new()
        // General routes
        .nest("authorize", auth::init_routes())
        .nest("users", users::init_routes())
        // Specific game routes
        .nest("osu", osu::init_routes())
}

pub fn convert_result<T>(input: RepoResult<T>, model_type: &str) -> Result<T, ApiError>
where
    T: Serialize,
{
    return match input {
        Ok(value) => match value {
            Some(value) => Ok(value),
            None => Err(ApiError::NotFound(model_type.to_string())),
        },
        Err(e) => Err(ApiError::Database(e)),
    };
}

// Custom error

#[derive(Serialize, Deserialize)]
struct ApiErrorWrapper<'a> {
    error: &'a str,
    description: &'a str,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database Error: {0}")]
    Database(#[from] RepoError),

    #[error("Not Found: {0}")]
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            // 4xx errors
            ApiError::Database(RepoError::Duplicate(..)) => (StatusCode::BAD_REQUEST, "duplicate"),
            ApiError::NotFound(..) => (StatusCode::NOT_FOUND, "not_found"),

            // 5xx errors
            ApiError::Database(..) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "database_internal_error")
            }
        };

        let body = Json(ApiErrorWrapper {
            error: error_message,
            description: &self.to_string(),
        });

        (status_code, body).into_response()
    }
}

// Custom Response
#[derive(Debug)]
struct ApiResponse<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
}

impl<T> Default for ApiResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body: None,
            status_code: StatusCode::OK,
        }
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.body {
            Some(body) => body,
            None => return (self.status_code).into_response(),
        };

        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &body) {
            // error!("Error serializing response body as JSON: {:?}", err);
            // handle logging
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        let bytes = bytes.into_inner().freeze();
        (self.status_code, bytes).into_response()
    }
}
