use std::str::FromStr;

use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::{OsuMap, OsuMappool};
use crate::models::osu::BeatmapMod;
use crate::repository::{to_object_id, Repo};
use crate::routes::{convert_result, ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router {
    Router::new()
        .route("", get(mappools_list).post(mappools_create))
        .route(
            "/:mappool_id",
            get(mappools_get)
                .patch(mappools_modify)
                .delete(mappools_delete),
        )
}

pub async fn mappools_get(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
) -> ApiResult<OsuMappool> {
    let mappool = repo.osu.tournaments.find_mappool_by_id(&mappool_id).await;

    let mappool = match convert_result(mappool, "mappool") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new().status_code(StatusCode::OK).body(mappool))
}

pub async fn mappools_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuMappool>> {
    let mappools = repo.osu.tournaments.list_mappools().await;

    let mappools = match convert_result(mappools, "mappools") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    if mappools.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(mappools))
}

pub async fn mappools_create() -> ApiResult<OsuMappool> {
    todo!();
}

pub async fn mappools_modify() -> ApiResult<OsuMappool> {
    todo!();
}

pub async fn mappools_delete(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
) -> ApiResult<()> {
    let mappool = repo
        .osu
        .tournaments
        .delete_tournament(doc! {
            "_id": to_object_id(&mappool_id)
        })
        .await;

    let mappool = match convert_result(mappool, "mappool") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddMapRequest {
    pub map_id: String,
    pub modifier: String,
}

pub async fn mappools_add_map(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
    Json(data): Json<AddMapRequest>,
) -> ApiResult<()> {
    let mappool = repo
        .osu
        .tournaments
        .find_mappool(doc! {
            "_id": to_object_id(&mappool_id)
        })
        .await;

    let mut mappool = match convert_result(mappool, "mappool") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    mappool.maps.push(OsuMap {
        osu_beatmap_id: data.map_id.parse::<i64>().unwrap(),
        modifier: BeatmapMod::from_str(&data.modifier).unwrap(),
    });

    // TODO: Replace mappool

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}

pub async fn maps_remove_map(
    State(repo): State<Repo>,
    Path((mappool_id, map_id)): Path<(String, String)>,
) -> ApiResult<()> {
    let mappool = repo
        .osu
        .tournaments
        .find_mappool(doc! {
            "_id": to_object_id(&mappool_id)
        })
        .await;

    let mut mappool = match convert_result(mappool, "mappool") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let map_pos = mappool
        .get_map_position(map_id.parse::<i64>().unwrap())
        .await;

    if map_pos.is_none() {
        return Err(ApiError::MapNotFound);
    }

    mappool.maps.remove(map_pos.unwrap());

    // TODO: Replace mappool

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
