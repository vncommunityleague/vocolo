use actix_web::error::HttpError;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("mappools")
            .service(mappools_get)
            .service(mappools_list)
            .service(mappools_post)
            .service(mappools_patch)
            .service(mappools_delete),
    );
}

#[get("{id}")]
pub async fn mappools_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn mappools_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn mappools_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn mappools_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn mappools_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}
