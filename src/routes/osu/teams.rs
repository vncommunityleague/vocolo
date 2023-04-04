use actix_web::error::HttpError;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("teams")
            .service(teams_get)
            .service(teams_list)
            .service(teams_post)
            .service(teams_patch)
            .service(teams_delete),
    );
}

#[get("{id}")]
pub async fn teams_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn teams_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn teams_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn teams_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn teams_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}
