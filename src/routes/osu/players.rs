use actix_web::error::HttpError;
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, HttpResponse};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("players")
            .service(players_get)
            .service(players_list)
            .service(players_post)
            .service(players_patch)
            .service(players_delete),
    );
}

#[get("{id}")]
pub async fn players_get() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[get("")]
pub async fn players_list() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[post("")]
pub async fn players_post() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[patch("{id}")]
pub async fn players_patch() -> Result<HttpResponse, HttpError> {
    todo!();
}

#[delete("{id}")]
pub async fn players_delete() -> Result<HttpResponse, HttpError> {
    todo!();
}
