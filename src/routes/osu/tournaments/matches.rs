use axum::Router;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::repository::Repo;
use crate::routes::{ApiError, ApiResult};

pub fn init_routes() -> Router {
    Router::new()
        .route("", get(matches_list).post(matches_post))
        .route("/:match_id", get(matches_get).patch(matches_patch).delete(matches_delete))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchCreationData {
    pub title: String,
    pub slug: String,
}

#[get("{id}")]
pub async fn matches_get() -> ApiResult {
    todo!();
}

#[get("")]
pub async fn matches_list() -> ApiResult {
    todo!();
}

#[post("")]
pub async fn matches_post(repo: Data<Repo>, data: web::Json<MatchCreationData>) -> ApiResult {
    todo!();
}

#[patch("{id}")]
pub async fn matches_patch() -> ApiResult {
    todo!();
}

#[delete("{id}")]
pub async fn matches_delete() -> ApiResult {
    todo!();
}
