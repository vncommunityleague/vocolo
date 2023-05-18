use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use crate::repository::RepoError;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

mod auth;
mod osu;
mod users;

pub type ApiResult<T> = Result<T, ApiError>;

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
            ApiError::InternalServerError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error"),
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found"),
            ApiError::TournamentNotFound => (StatusCode::NOT_FOUND, "tournament_not_found"),
            ApiError::TournamentTeamNotFound => (StatusCode::NOT_FOUND, "tournament_team_not_found"),
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

pub fn init_routes() -> Router {
    Router::new()
        // General routes
        .nest("/authorize", auth::init_routes())
        .nest("/users", users::init_routes())
        // Specific game routes
        .nest("/osu", osu::init_routes())
}
