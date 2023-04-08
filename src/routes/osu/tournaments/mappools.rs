use actix_web::web::{Data, ServiceConfig};
use actix_web::{delete, get, patch, post, web, HttpResponse};

use crate::repository::Repo;
use crate::routes::ApiError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(mappools_get);
    cfg.service(mappools_list);
    cfg.service(mappools_post);
    cfg.service(mappools_patch);
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
pub async fn mappools_patch() -> Result<HttpResponse, ApiError> {
    todo!();
}

#[delete("{tournament_id}/mappools/{mappool_id}")]
pub async fn mappools_delete() -> Result<HttpResponse, ApiError> {
    todo!();
}

// TODO add methods for adding and removing maps from mappools
