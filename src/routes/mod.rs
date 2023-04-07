use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use derive_more::{Display, Error};
use std::fmt::{Display, Formatter};

mod auth;
mod osu;
mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("authorize").configure(auth::config));
    cfg.service(web::scope("osu").configure(osu::config));
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Invalid input: {}", input)]
    InvalidInput { input: String },
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InvalidInput { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
