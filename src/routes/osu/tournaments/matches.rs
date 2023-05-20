use axum::{
    extract::{Path, Query},
    Json,
    Router, routing::{delete, get, post, put},
};
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::models::osu::tournaments::OsuMatch;

use crate::repository::Repo;
use crate::routes::{ApiError, ApiResponse, ApiResult, convert_result};

pub fn init_routes() -> Router {
    Router::new()
        .route("", get(matches_list).post(matches_create))
        .route(
            "/:match_id",
            get(matches_get).patch(matches_modify).delete(matches_delete),
        )
}

pub async fn matches_get(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<OsuMatch> {
    let game_match = repo
        .osu
        .matches
        .find_match_by_id(&match_id)
        .await;

    let game_match = match convert_result(game_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(game_match)
    )
}

pub async fn matches_list() -> ApiResult<OsuMatch> {
    todo!();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchCreationData {
    pub title: String,
    pub slug: String,
}

pub async fn matches_create(repo: Data<Repo>, data: web::Json<MatchCreationData>) -> ApiResult<OsuMatch> {
    todo!();
}

pub async fn matches_modify() -> ApiResult<OsuMatch> {
    todo!();
}

pub async fn matches_delete() -> ApiResult<OsuMatch> {
    todo!();
}
