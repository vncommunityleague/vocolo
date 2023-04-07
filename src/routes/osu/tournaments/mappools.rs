use crate::repository::Repo;
use crate::routes::ApiError;
use actix_web::web::{Data, ServiceConfig};
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

#[get("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_get(repo: Data<Repo>) -> Result<HttpResponse, ApiError> {
    todo!();
}

#[get("")]
pub async fn mappools_list() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[post("")]
pub async fn mappools_post() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[patch("{id}")]
pub async fn mappools_patch() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[delete("{id}")]
pub async fn mappools_delete() -> Result<HttpResponse, ApiError> {
    todo!();
}
