use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::ApiError;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchCreationData {
    pub title: String,
    pub slug: String,
}

#[get("{id}")]
pub async fn matches_get() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[get("")]
pub async fn matches_list() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[post("")]
pub async fn matches_post(
    repo: Data<Repo>,
    data: web::Json<MatchCreationData>,
) -> Result<HttpResponse, ApiError> {
    todo!();
}

#[patch("{id}")]
pub async fn matches_patch() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[delete("{id}")]
pub async fn matches_delete() -> Result<HttpResponse, ApiError> {
    todo!();
}
