use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::OsuMatch;
use crate::repository::{to_object_id, Repo};
use crate::routes::{convert_result, ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("", get(matches_list).post(matches_create))
        .route(
            "/:match_id",
            get(matches_get)
                .patch(matches_modify)
                .delete(matches_delete),
        )
}

pub async fn matches_get(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<OsuMatch> {
    let game_match = repo.osu.tournaments.find_match_by_id(&match_id).await;

    let game_match = match convert_result(game_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(game_match))
}

pub async fn matches_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuMatch>> {
    let game_matches = repo.osu.tournaments.list_matches().await;

    let game_matches = match convert_result(game_matches, "matches") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    if game_matches.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(game_matches))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchCreationData {
    pub title: String,
    pub slug: String,
}

pub async fn matches_create(
    State(repo): State<Repo>,
    Json(data): Json<MatchCreationData>,
) -> ApiResult<OsuMatch> {
    let game_match = repo
        .osu
        .tournaments
        .create_match(doc! {
            "title": data.title,
            "slug": data.slug,
        })
        .await;

    let game_match = match convert_result(game_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::CREATED)
        .body(game_match))
}

pub async fn matches_modify() -> ApiResult<OsuMatch> {
    todo!();
}

pub async fn matches_delete(
    State(repo): State<Repo>,
    Path(match_id): Path<String>,
) -> ApiResult<OsuMatch> {
    let game_match = repo
        .osu
        .tournaments
        .delete_tournament(doc! {
            "_id": to_object_id(&match_id)
        })
        .await;

    let game_match = match convert_result(game_match, "match") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
