use crate::repository::RepoError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

mod auth;
mod osu;
mod users;

pub type ApiResult = Result<HttpResponse, ApiError>;

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

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::Duplicate { .. } => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UserNotFound
            | ApiError::TournamentNotFound
            | ApiError::TournamentTeamNotFound
            | ApiError::MapNotFound
            | ApiError::MappoolNotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ApiErrorWrapper {
            error: match self {
                ApiError::Duplicate { .. } => "duplicate",
                ApiError::InternalServerError { .. } => "internal_server_error",
                ApiError::UserNotFound => "user_not_found",
                ApiError::TournamentNotFound => "tournament_not_found",
                ApiError::TournamentTeamNotFound => "tournament_team_not_found",
                ApiError::MapNotFound => "map_not_found",
                ApiError::MappoolNotFound => "mappool_not_found",
            },
            description: &self.to_string(),
        })
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("authorize").configure(auth::config));
    cfg.service(web::scope("osu").configure(osu::config));
    cfg.service(web::scope("users").configure(users::config));
}
