use crate::repository::RepoError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use bytes::BytesMut;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

mod auth;
mod osu;
mod users;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "duplicate key: {}", key)]
    Duplicate { key: String },

    #[display(fmt = "internal server error: {}", message)]
    InternalServerError { message: String },

    #[display(fmt = "user not found")]
    UserNotFound,

    #[display(fmt = "tournament not found")]
    TournamentNotFound,

    #[display(fmt = "tournament team not found")]
    TournamentTeamNotFound,

    #[display(fmt = "map not found")]
    MapNotFound,

    #[display(fmt = "mappool not found")]
    MappoolNotFound,
}

#[derive(Serialize, Deserialize)]
struct ApiErrorWrapper<'a> {
    error: &'a str,
    description: &'a str,
}

impl ApiError {
    pub fn from_repo_error(error: RepoError) -> Self {
        match error {
            RepoError::AlreadyExist { key } => ApiError::Duplicate { key },
            RepoError::QueryFatal { message } => ApiError::InternalServerError { message },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::Duplicate { .. } => (StatusCode::BAD_REQUEST, "duplicate"),
            ApiError::InternalServerError { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error")
            }
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found"),
            ApiError::TournamentNotFound => (StatusCode::NOT_FOUND, "tournament_not_found"),
            ApiError::TournamentTeamNotFound => {
                (StatusCode::NOT_FOUND, "tournament_team_not_found")
            }
            ApiError::MapNotFound => (StatusCode::NOT_FOUND, "map_not_found"),
            ApiError::MappoolNotFound => (StatusCode::NOT_FOUND, "mappool_not_found"),
        };

        let body = ApiErrorWrapper {
            error: error_message,
            description: &self.to_string(),
        };

        (status_code, body).into_response()
    }
}

pub fn get_option_from_query<T>(input: Result<Option<T>, RepoError>) -> Option<T> {
    match &input {
        Ok(value) => value,
        Err(error) => ApiError::from_repo_error(error),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        // General routes
        .nest("/authorize", auth::init_routes())
        .nest("/users", users::init_routes())
        // Specific game routes
        .nest("/osu", osu::init_routes())
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

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

    pub fn body(&mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn status_code(&mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(&self) -> Response {
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

        (self.status_code, bytes).into_response()
    }
}
