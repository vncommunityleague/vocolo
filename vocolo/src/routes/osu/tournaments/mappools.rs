use std::str::FromStr;

use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use axum::routing::{delete, post};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::models::osu::tournaments::{OsuMap, OsuMappool};
use crate::models::osu::BeatmapMod;
use crate::repository::model::ModelExt;
use crate::repository::{to_object_id, Repo};
use crate::routes::{ApiError, ApiResponse, ApiResult};

pub fn init_routes() -> Router<Repo> {
    Router::new()
        .route("/", get(mappools_list).post(mappools_create))
        .route(
            "/:mappool_id",
            get(mappools_get)
                .patch(mappools_update)
                .delete(mappools_delete),
        )
        .route(
            "/:mappool_id/maps",
            post(mappools_add_map),
        )
        .route(
            "/:mappool_id/maps/:map_id",
            delete(mappools_remove_map),
        )
}

pub async fn mappools_get(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
) -> ApiResult<OsuMappool> {
    let osu_mappool = OsuMappool::find_by_id(
        repo.osu.tournaments.mappools_col,
        &to_object_id(&mappool_id),
    )
    .await
    .map_err(ApiError::Database)?;

    let osu_mappool = match osu_mappool {
        Some(value) => value,
        None => return Err(ApiError::NotFound("osu_mappool".to_string())),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_mappool))
}

pub async fn mappools_list(State(repo): State<Repo>) -> ApiResult<Vec<OsuMappool>> {
    let osu_mappools = OsuMappool::list(repo.osu.tournaments.mappools_col)
        .await
        .map_err(ApiError::Database)?;

    if osu_mappools.is_empty() {
        return Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT));
    }

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_mappools))
}

#[derive(Serialize, Deserialize)]
pub struct MappoolCreationRequest {
    name: String,

    #[serde(default)]
    maps: Vec<OsuMap>,
}

pub async fn mappools_create(
    State(repo): State<Repo>,
    Json(data): Json<MappoolCreationRequest>,
) -> ApiResult<OsuMappool> {
    let mut osu_mappool = OsuMappool::default();
    osu_mappool.name = data.name;

    let osu_mappool = OsuMappool::create(repo.osu.tournaments.mappools_col, osu_mappool)
        .await
        .map_err(ApiError::Database)?;

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_mappool))
}

#[derive(Serialize, Deserialize)]
pub struct MappoolUpdateRequest {
    name: Option<String>,

    #[serde(default)]
    maps: Vec<OsuMap>,
}

pub async fn mappools_update(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
    Json(data): Json<MappoolCreationRequest>,
) -> ApiResult<OsuMappool> {
    let osu_mappool = OsuMappool::find_one_and_update(
        repo.osu.tournaments.mappools_col,
        doc! {"_id": to_object_id(&mappool_id)},
        doc! {"$set": bson::to_document(&data).unwrap()},
    )
    .await
    .map_err(ApiError::Database)?;

    let osu_mappool = match osu_mappool {
        Some(value) => value,
        None => return Err(ApiError::NotFound("osu_mappool".to_string())),
    };

    Ok(ApiResponse::new()
        .status_code(StatusCode::OK)
        .body(osu_mappool))
}

pub async fn mappools_delete(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
) -> ApiResult<()> {
    let _ = OsuMappool::delete_one(
        repo.osu.tournaments.mappools_col,
        doc! {"_id": to_object_id(&mappool_id)},
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddMapRequest {
    pub map_id: i64,
    pub modifier: Vec<String>,
}

pub async fn mappools_add_map(
    State(repo): State<Repo>,
    Path(mappool_id): Path<String>,
    Json(data): Json<AddMapRequest>,
) -> ApiResult<()> {
    let _ = OsuMappool::find_one_and_update(
        repo.osu.tournaments.mappools_col,
        doc! {"_id": to_object_id(&mappool_id)},
        doc! {
            "$push": {
                "maps": bson::to_document(&OsuMap {
                    osu_beatmap_id: data.map_id,
                    modifier: data.modifier.into_iter().map(|x| BeatmapMod::from_str(&x).unwrap()).collect::<Vec<BeatmapMod>>(),
                }).unwrap()
            }
        },
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}

pub async fn mappools_remove_map(
    State(repo): State<Repo>,
    Path((mappool_id, map_id)): Path<(String, String)>,
) -> ApiResult<()> {
    let _ = OsuMappool::find_one_and_update(
        repo.osu.tournaments.mappools_col,
        doc! {"_id": to_object_id(&mappool_id)},
        doc! {
            "$pull": {
                "maps": bson::to_document(&OsuMap {
                    osu_beatmap_id: map_id.parse::<i64>().unwrap(),
                    modifier: vec![],
                }).unwrap()
            }
        },
    )
    .await
    .map_err(ApiError::Database)?;

    Ok(ApiResponse::new().status_code(StatusCode::NO_CONTENT))
}
