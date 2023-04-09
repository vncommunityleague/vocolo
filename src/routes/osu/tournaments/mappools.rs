use crate::models::osu::tournaments::OsuMap;
use crate::models::osu::BeatmapMod;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::repository::Repo;
use crate::routes::ApiError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(mappools_get);
    cfg.service(mappools_list);
    cfg.service(mappools_post);
    cfg.service(mappools_modify);
    cfg.service(mappools_delete);
}

#[get("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_get(
    repo: Data<Repo>,
    info: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let mappool_id = &path.1;
    let tournament = repo.osu.find_tournament_by_id_or_slug(tournament_id).await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();
    let mappool = tournament.get_mappool(mappool_id.to_string()).await;

    if mappool.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    Ok(HttpResponse::Ok().json(mappool.unwrap()))
}

#[get("{tournament_id}/mappools")]
pub async fn mappools_list(
    repo: Data<Repo>,
    info: web::Path<(String,)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let tournament = repo.osu.find_tournament_by_id_or_slug(tournament_id).await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let tournament = tournament.unwrap();

    Ok(HttpResponse::Ok().json(tournament.mappools))
}

#[post("{tournament_id}/mappools")]
pub async fn mappools_post() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[patch("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_modify() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[delete("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_delete() -> Result<HttpResponse, ApiError> {
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
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let mappool_id = &path.1;
    let tournament = repo.osu.find_tournament_by_id_or_slug(tournament_id).await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let mappool_pos = tournament
        .get_mappool_position(mappool_id.to_string())
        .await;

    if mappool_pos.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    tournament.mappools[mappool_pos.unwrap()].maps.push(OsuMap {
        osu_beatmap_id: data.map_id.parse::<i64>().unwrap(),
        modifier: BeatmapMod::from_str(&data.modifier).unwrap(),
    });

    repo.osu
        .replace_tournament(&tournament.info.slug, tournament.clone())
        .await
        .unwrap();

    Ok(HttpResponse::NoContent().finish())
}

#[delete("{tournament_id}/mappools/{mappool_id}/maps/{map_id}")]
pub async fn maps_remove_map(
    repo: Data<Repo>,
    info: web::Path<(String, String, String)>,
) -> Result<HttpResponse, ApiError> {
    let path = info.into_inner();
    let tournament_id = &path.0;
    let mappool_id = &path.1;
    let map_id = &path.2;
    let tournament = repo.osu.find_tournament_by_id_or_slug(tournament_id).await;

    if tournament.is_none() {
        return Err(ApiError::TournamentNotFound);
    }

    let mut tournament = tournament.unwrap();
    let mappool_pos = tournament
        .get_mappool_position(mappool_id.to_string())
        .await;

    if mappool_pos.is_none() {
        return Err(ApiError::MappoolNotFound);
    }

    let mappool = &mut tournament.mappools[mappool_pos.unwrap()];
    let map_pos = mappool
        .get_map_position(map_id.parse::<i64>().unwrap())
        .await;

    if map_pos.is_none() {
        return Err(ApiError::MapNotFound);
    }

    mappool.maps.remove(map_pos.unwrap());

    repo.osu
        .replace_tournament(&tournament.info.slug, tournament.clone())
        .await
        .unwrap();

    Ok(HttpResponse::NoContent().finish())
}
