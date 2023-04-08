use actix_web::{HttpResponse, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

mod auth;
mod osu;
mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("authorize").configure(auth::config));
    cfg.service(web::scope("osu").configure(osu::config));
    cfg.service(web::scope("users").configure(users::config));
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Invalid input: {}", input)]
    InvalidInput { input: String },

    #[display(fmt = "Tournament not found")]
    TournamentNotFound,

    #[display(fmt = "Team not found")]
    TeamNotFound,

    #[display(fmt = "User not found")]
    UserNotFound
}

#[derive(Serialize, Deserialize)]
struct ApiErrorWrapper<'a> {
    error: &'a str,
    description: &'a str,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InvalidInput { .. } => StatusCode::BAD_REQUEST,
            ApiError::TournamentNotFound => StatusCode::NOT_FOUND,
            ApiError::TeamNotFound => StatusCode::NOT_FOUND,
            ApiError::UserNotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ApiErrorWrapper {
            error: match *self {
                ApiError::InvalidInput { .. } => "invalid_input",
                ApiError::TournamentNotFound => "tournament_not_found",
                ApiError::TeamNotFound => "team_not_found",
                ApiError::UserNotFound => "user_not_found",
            },
            description: &self.to_string(),
        })
    }
}
