use crate::models::osu::tournaments::{OsuMap, OsuMappool};
use crate::models::osu::BeatmapMod;
use axum::extract::State;
use axum::Router;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::repository::Repo;
use crate::routes::{get_option_from_query, ApiError, ApiResult};

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
    Path(tournament_id, mappool_id): Path<(String, String)>,
) -> ApiResult<OsuMappool> {
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(&tournament_id)
        .await;

    match &get_option_from_query(tournament) {
        Some(value) => tournament = value,
        None => Err(ApiError::TournamentNotFound),
    };

    let mappool = tournament.get_mappool(mappool_id.to_string()).await;

    match &mappool {
        Ok(value) => mappool = value,
        None => Err(ApiError::MappoolNotFound),
    }

    if mappool.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    Ok(HttpResponse::Ok().json(mappool.unwrap().1))
}

pub async fn mappools_list(repo: Data<Repo>, info: web::Path<(String,)>) -> ApiResult {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::from_repo_error(tournament.err().unwrap()));
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    Ok(HttpResponse::Ok().json(tournament.mappools))
}

pub async fn mappools_create() -> ApiResult {
    todo!();
}

pub async fn mappools_modify() -> ApiResult {
    todo!();
}

#[delete("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_delete() -> ApiResult {
    todo!();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddMapRequest {
    pub map_id: String,
    pub modifier: String,
}

#[post("{tournament_id}/mappools/{mappool_id}/maps")]
pub async fn mappools_add_map(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
    data: web::Json<AddMapRequest>,
) -> ApiResult {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let mappool_id = &path.1;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let mappool = tournament.get_mappool(mappool_id.to_string()).await;

    if mappool.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    let mappool = mappool.unwrap();
    let mappool_pos = mappool.0;

    tournament.mappools[mappool_pos].maps.push(OsuMap {
        osu_beatmap_id: data.map_id.parse::<i64>().unwrap(),
        modifier: BeatmapMod::from_str(&data.modifier).unwrap(),
    });

    repo.osu
        .tournaments
        .replace_tournament(&tournament.info.slug, tournament.clone())
        .await
        .unwrap();

    Ok(HttpResponse::NoContent().finish())
}

#[delete("{tournament_id}/mappools/{mappool_id}/maps/{map_id}")]
pub async fn maps_remove_map(
    repo: Data<Repo>,
    info: web::Path<(String, String, String)>,
) -> ApiResult {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let mappool_id = &path.1;
    let map_id = &path.2;
    let tournament = repo
        .osu
        .tournaments
        .find_tournament_by_id_or_slug(tournament_id)
        .await;

    if tournament.is_err() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let mappool_pos = tournament.get_mappool(mappool_id.to_string()).await;

    if mappool_pos.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    let mappool = &mut tournament.mappools[mappool_pos.unwrap().0];
    let map_pos = mappool
        .get_map_position(map_id.parse::<i64>().unwrap())
        .await;

    if map_pos.is_none() {
        return Err(ApiError::MapNotFound);
    }

    mappool.maps.remove(map_pos.unwrap());

    repo.osu
        .tournaments
        .replace_tournament(&tournament.info.slug, tournament.clone())
        .await
        .unwrap();

    Ok(HttpResponse::NoContent().finish())
}
