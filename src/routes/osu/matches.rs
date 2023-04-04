use actix_web::error::HttpError;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("matches")
            .service(matches_get)
            .service(matches_list)
            .service(matches_post)
            .service(matches_patch)
            .service(matches_delete),
    );
}

#[get("{id}")]
pub async fn matches_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn matches_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn matches_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn matches_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn matches_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}
